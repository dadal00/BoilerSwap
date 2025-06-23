use super::{
    lock::{freeze_account, unfreeze_account},
    models::{Account, Action, RedisAccount, RedisAction, Token},
    redis::{create_redis_account, get_redis_account},
    sessions::{create_session, create_temporary_session, generate_cookie, get_cookie},
    twofactor::{CODE_REGEX, generate_code},
    verify::{
        validate_account, validate_api_token, validate_email, validate_password, verify_token,
    },
};
use crate::{AppError, state::AppState};
use axum::{
    Json,
    extract::{ConnectInfo, Request, State},
    http::{StatusCode, header::HeaderMap},
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
    if validate_api_token(headers, &state.config.api_token) {
        return Ok(next.run(request).await);
    }

    Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response())
}

pub async fn forgot_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Token>,
) -> Result<impl IntoResponse, AppError> {
    if validate_email(&payload.token).is_err() {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
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
            600,
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
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Token>,
) -> Result<impl IntoResponse, AppError> {
    let (result, redis_action, id) = match verify_token(state.clone(), headers).await? {
        Some((a, b, c)) => (a, b, c),
        None => {
            return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
        }
    };

    if redis_action == RedisAction::Update && validate_password(&payload.token).is_err() {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    if (redis_action == RedisAction::Auth || redis_action == RedisAction::Forgot)
        && (payload.token.len() != 6 || !CODE_REGEX.is_match(&payload.token))
    {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    let redis_account = match get_redis_account(
        state.clone(),
        &result,
        &redis_action,
        &id,
        &payload.token,
        RedisAction::LockedTemporary,
    )
    .await?
    {
        Some(account) => account,
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
                600,
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
            3600,
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
    if payload.action == Action::Forgot {
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
    )
    .await?
    {
        Some(account) => account,
        None => {
            return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
        }
    };

    Ok((
        StatusCode::OK,
        create_temporary_session(state.clone(), &None, &redis_account, RedisAction::Auth, 600)
            .await?,
    )
        .into_response())
}
