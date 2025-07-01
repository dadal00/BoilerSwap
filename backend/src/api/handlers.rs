use super::{
    database::insert_item,
    lock::{freeze_account, unfreeze_account},
    models::{Account, Action, ItemPayload, RedisAccount, RedisAction, Token},
    redis::{create_redis_account, get_redis_account, incr_failed_attempts, remove_id, try_get},
    sessions::{create_session, create_temporary_session, generate_cookie, get_cookie},
    twofactor::{CODE_REGEX, generate_code},
    utilities::{get_hashed_ip, get_key},
    verify::{
        CODE_LENGTH, validate_account, validate_api_token, validate_email, validate_item,
        validate_password, verify_token,
    },
};
use crate::{AppError, state::AppState};
use axum::{
    Json,
    extract::{ConnectInfo, Request, State},
    http::{
        StatusCode,
        header::{HeaderMap, ORIGIN},
    },
    middleware::Next,
    response::IntoResponse,
};
use redis::AsyncTypedCommands;
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

pub async fn default_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    info!("Received!");
    "Hello from Axum!"
}

pub async fn api_token_check(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let origin = headers.get(ORIGIN);

    if origin.is_none() {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    if origin.expect("is_none failed").as_bytes() != state.config.svelte_url.as_bytes() {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    if validate_api_token(headers) {
        return Ok(next.run(request).await);
    }

    Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response())
}

pub async fn forgot_handler(
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Token>,
) -> Result<impl IntoResponse, AppError> {
    if validate_email(&payload.token).is_err() {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }

    let hashed_ip = get_hashed_ip(&headers, address.ip());

    let forgot_key = get_key(RedisAction::LockedForgot, &hashed_ip);
    let failed_verify_key = get_key(RedisAction::LockedVerify, &hashed_ip);

    if let Some(attempts) = try_get(state.clone(), &failed_verify_key, &payload.token).await? {
        if attempts.parse::<u8>()? >= state.config.verify_max_attempts {
            return Ok((StatusCode::UNAUTHORIZED, "Rate Limited").into_response());
        }
    }

    let redis_account = RedisAccount {
        email: payload.token,
        action: Action::Forgot,
        code: generate_code().clone(),
        issued_timestamp: None,
        password_hash: None,
    };

    Ok((
        StatusCode::OK,
        create_temporary_session(
            state.clone(),
            &None,
            &redis_account,
            RedisAction::Forgot,
            &Some(forgot_key),
        )
        .await?,
    )
        .into_response())
}

pub async fn delete_handler(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let id = get_cookie(&headers, RedisAction::Session.as_ref());

    if id.is_some() {
        state
            .redis_connection_manager
            .clone()
            .del(format!(
                "{}:{}",
                RedisAction::Session.as_ref(),
                id.expect("is_none failed")
            ))
            .await?;
    }

    Ok((StatusCode::OK, generate_cookie("", "", 0)).into_response())
}

pub async fn verify_handler(
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Token>,
) -> Result<impl IntoResponse, AppError> {
    let (result, redis_action, id) = match verify_token(state.clone(), headers.clone()).await? {
        Some((a, pending_redis_action, c)) => {
            if pending_redis_action == RedisAction::Session {
                return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
            }
            (a, pending_redis_action, c)
        }
        None => {
            return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
        }
    };

    if redis_action == RedisAction::Update && validate_password(&payload.token).is_err() {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    if (redis_action == RedisAction::Auth || redis_action == RedisAction::Forgot)
        && (payload.token.len() != *CODE_LENGTH || !CODE_REGEX.is_match(&payload.token))
    {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    let hashed_ip = get_hashed_ip(&headers, address.ip());

    let forgot_key = get_key(RedisAction::LockedForgot, &hashed_ip);
    let failed_verify_key = get_key(RedisAction::LockedVerify, &hashed_ip);
    let failed_auth_key = get_key(RedisAction::LockedAuth, &hashed_ip);

    let redis_account = match get_redis_account(
        state.clone(),
        &result,
        &redis_action,
        &id,
        &payload.token,
        RedisAction::LockedTemporary,
        &failed_verify_key,
    )
    .await?
    {
        Some(account) => {
            remove_id(state.clone(), &failed_verify_key, &account.email).await?;
            remove_id(state.clone(), &failed_auth_key, &account.email).await?;
            remove_id(state.clone(), &forgot_key, &account.email).await?;
            account
        }
        None => {
            return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
        }
    };

    if redis_action == RedisAction::Forgot {
        freeze_account(state.clone(), &redis_account.email).await?;

        return Ok((
            StatusCode::OK,
            create_temporary_session(
                state.clone(),
                &result,
                &redis_account,
                RedisAction::Update,
                &None,
            )
            .await?,
        )
            .into_response());
    }

    if redis_action == RedisAction::Update {
        unfreeze_account(state.clone(), &redis_account.email, &payload.token).await?;
    }

    Ok((
        StatusCode::OK,
        create_session(
            state.clone(),
            &redis_account,
            RedisAction::Session,
            RedisAction::SessionStore,
        )
        .await?,
    )
        .into_response())
}

pub async fn authenticate_handler(
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Account>,
) -> Result<impl IntoResponse, AppError> {
    let hashed_ip = get_hashed_ip(&headers, address.ip());

    let failed_auth_key = get_key(RedisAction::LockedAuth, &hashed_ip);

    if let Some(attempts) = try_get(state.clone(), &failed_auth_key, &payload.email).await? {
        if attempts.parse::<u8>()? >= state.config.auth_max_attempts {
            return Ok((StatusCode::UNAUTHORIZED, "Rate Limited").into_response());
        }
    }

    if payload.action == Action::Forgot {
        incr_failed_attempts(
            state.clone(),
            &failed_auth_key,
            &payload.email,
            &state.config.auth_lock_duration_seconds,
            &state.config.auth_max_attempts,
        )
        .await?;
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    if let Err(e) = validate_account(&payload.email, &payload.password) {
        return Ok((StatusCode::BAD_REQUEST, e).into_response());
    }

    let redis_account = match create_redis_account(
        state.clone(),
        payload.action,
        &payload.email,
        &payload.password,
        &failed_auth_key,
    )
    .await?
    {
        Some(account) => {
            remove_id(state.clone(), &failed_auth_key, &payload.email).await?;
            account
        }
        None => {
            return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
        }
    };

    Ok((
        StatusCode::OK,
        create_temporary_session(
            state.clone(),
            &None,
            &redis_account,
            RedisAction::Auth,
            &None,
        )
        .await?,
    )
        .into_response())
}

pub async fn post_item_handler(
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ItemPayload>,
) -> Result<impl IntoResponse, AppError> {
    match verify_token(state.clone(), headers.clone()).await? {
        Some((_, redis_action, _)) => {
            if redis_action != RedisAction::Session {
                return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
            }
        }
        None => {
            return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
        }
    };

    if let Err(e) = validate_item(&payload.title, &payload.description) {
        return Ok((StatusCode::BAD_REQUEST, e).into_response());
    }

    insert_item(state.clone(), payload).await?;

    Ok((StatusCode::OK).into_response())
}
