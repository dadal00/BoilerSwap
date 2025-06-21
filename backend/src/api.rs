use crate::{
    AppError,
    database::{check_lock, get_user, insert_user, unlock_account, update_lock},
    redis::{
        delete_all_sessions, insert_id, insert_session, is_temporarily_locked, remove_id, try_get,
    },
    state::AppState,
};
use argon2::{
    Algorithm::Argon2id, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
    Version::V0x13, password_hash::SaltString,
};
use axum::{
    Json,
    extract::{ConnectInfo, Request, State},
    http::{
        HeaderValue, StatusCode,
        header::{AUTHORIZATION, HeaderMap, SET_COOKIE},
    },
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use chrono::{Duration as chronoDuration, Utc};
use cookie::{Cookie, CookieJar as cookieCookieJar, SameSite::Strict, time::Duration};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use once_cell::sync::Lazy;
use rand::{Rng, rngs::OsRng, thread_rng};
use redis::AsyncTypedCommands;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use strum_macros::{AsRefStr, EnumString};
use tokio::task::spawn_blocking;
use tracing::{debug, info, warn};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(Clone)]
pub enum Action {
    Login,
    Signup,
    Forgot,
}

#[derive(Debug, EnumString, AsRefStr, PartialEq)]
pub enum RedisAction {
    #[strum(serialize = "auth_id")]
    Auth,

    #[strum(serialize = "authenticating")]
    Authenticating,

    #[strum(serialize = "forgot_id")]
    Forgot,

    #[strum(serialize = "recovering")]
    Recovering,

    #[strum(serialize = "locked_timestamp")]
    LockedTime,

    #[strum(serialize = "session_id")]
    Session,

    #[strum(serialize = "temporary_lock")]
    LockedTemporary,

    #[strum(serialize = "update")]
    Update,

    #[strum(serialize = "updating")]
    Updating,

    #[strum(serialize = "sessions")]
    SessionStore,
}

static COOKIES_TO_CLEAR: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        RedisAction::Session.as_ref(),
        RedisAction::Forgot.as_ref(),
        RedisAction::Update.as_ref(),
        RedisAction::Auth.as_ref(),
    ]
});

#[derive(Serialize, Deserialize)]
pub struct Account {
    email: String,
    password: String,
    action: Action,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RedisAccount {
    pub email: String,
    action: Action,
    code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    issued_timestamp: Option<i64>,
    pub password_hash: Option<String>,
}

#[derive(Deserialize)]
pub struct Token {
    token: String,
}

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.+@purdue\.edu$").unwrap());
static AUTH_CODE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+$").unwrap());

pub static CLEARED_COOKIES: Lazy<cookieCookieJar> = Lazy::new(|| {
    let mut jar = cookieCookieJar::new();

    for &old_cookie in COOKIES_TO_CLEAR.iter() {
        let expired = Cookie::build(old_cookie)
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(Strict)
            .max_age(Duration::seconds(0));
        jar.add(expired);
    }

    jar
});

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

    freeze_account(state.clone(), &payload.token).await?;

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
            RedisAction::Recovering,
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
    let (result, redis_action, redis_action_secondary, id) =
        match verify_token(state.clone(), headers).await? {
            Some((a, b, c, d)) => (a, b, c, d),
            None => {
                return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
            }
        };

    if redis_action == RedisAction::Update && validate_password(&payload.token).is_err() {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    if (redis_action == RedisAction::Auth || redis_action == RedisAction::Forgot)
        && (payload.token.len() != 6 || !AUTH_CODE_REGEX.is_match(&payload.token))
    {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    let redis_account = match get_redis_account(
        state.clone(),
        &result,
        &redis_action,
        &id,
        &payload.token,
        &redis_action_secondary,
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
        return Ok((
            StatusCode::OK,
            create_temporary_session(
                state.clone(),
                &result,
                &redis_account,
                RedisAction::Update,
                RedisAction::Updating,
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
    if payload.action == Action::Forgot
        || try_get(
            state.clone(),
            RedisAction::Authenticating.as_ref(),
            &payload.email,
        )
        .await?
        .is_some()
    {
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
        create_temporary_session(
            state.clone(),
            &None,
            &redis_account,
            RedisAction::Auth,
            RedisAction::Authenticating,
            600,
        )
        .await?,
    )
        .into_response())
}

fn validate_email(email: &str) -> Result<(), &'static str> {
    if email.len() > 100 {
        return Err("Too many chars");
    }

    if !EMAIL_REGEX.is_match(email) {
        return Err("Email must be a Purdue address");
    }

    Ok(())
}

fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.len() > 100 {
        return Err("Too many chars");
    }

    if password.is_empty() {
        return Err("Password cannot be empty");
    }

    Ok(())
}

fn validate_account(email: &str, password: &str) -> Result<(), &'static str> {
    validate_email(email)?;

    validate_password(password)?;

    Ok(())
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let params = Params::new(65536, 3, 1, None).map_err(|e| {
        warn!("Failed to hash password: {}", e);
        AppError::Config(e.to_string())
    })?;

    let argon2 = Argon2::new(Argon2id, V0x13, params);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            warn!("Failed to hash password: {}", e);
            AppError::Config(e.to_string())
        })?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|e| {
        warn!("Failed to parse password hash: {}", e);
        AppError::Config(e.to_string())
    })?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

fn generate_code() -> String {
    let mut rng = thread_rng();

    format!("{:06}", rng.gen_range(0..1_000_000))
}

async fn send_code_email(
    state: Arc<AppState>,
    user_email: &str,
    code: &str,
) -> Result<(), AppError> {
    let email = Message::builder()
        .from(format!("BoilerSwap <{}>", state.config.from_email).parse()?)
        .to(user_email.parse()?)
        .subject("BoilerSwap Code")
        .body(format!("Your code is {}", code))?;

    let credentials = Credentials::new(
        state.config.from_email.to_string(),
        state.config.from_email_password.to_string(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&state.config.from_email_server)?
        .credentials(credentials)
        .build();

    mailer.send(email).await?;

    Ok(())
}

fn spawn_code_task(state: Arc<AppState>, email: String, token: String) {
    tokio::spawn(async move {
        if let Err(error) = send_code_email(state, &email, &token).await {
            match error {
                AppError::LettreAddress(msg) => {
                    debug!("Invalid email: {}", msg);
                }
                AppError::LettreTransport(msg) => {
                    debug!("Transport error: {}", msg);
                }
                other => {
                    warn!("Unexpected error: {:?}", other);
                }
            }
        }
    });
}

fn validate_api_token(headers: HeaderMap, real_api_token: &str) -> bool {
    if let Some(api_header) = headers.get(AUTHORIZATION) {
        if api_header
            .to_str()
            .is_ok_and(|api_token| api_token == real_api_token)
        {
            return true;
        }
    }

    if get_cookie(&headers, "api_token").unwrap_or_default() == real_api_token {
        return true;
    }

    false
}

fn generate_cookie(key: &str, value: &str, ttl: i64) -> HeaderMap {
    let mut jar = CLEARED_COOKIES.clone();

    let new_cookie = Cookie::build((key.to_owned(), value.to_owned()))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(Strict)
        .max_age(Duration::seconds(ttl));

    jar.add(new_cookie);

    let mut headers = HeaderMap::new();

    for cookie in jar.delta() {
        headers.append(
            SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
    }

    headers
}

fn get_cookie(headers: &HeaderMap, key: &str) -> Option<String> {
    CookieJar::from_headers(headers)
        .get(key)
        .map(|cookie| cookie.value().to_string())
}

async fn check_locks(
    state: Arc<AppState>,
    email: &str,
    issued_timestamp: i64,
) -> Result<bool, AppError> {
    if check_db_lock(state.clone(), email).await? {
        return Ok(true);
    }

    let locked_timestamp = try_get(state.clone(), RedisAction::LockedTime.as_ref(), email).await?;

    if locked_timestamp.is_some()
        && issued_timestamp
            < locked_timestamp
                .expect("is_some failed")
                .parse::<i64>()
                .unwrap_or(i64::MAX)
    {
        return Ok(true);
    }

    Ok(false)
}

async fn get_redis_account(
    state: Arc<AppState>,
    result: &Option<String>,
    redis_action: &RedisAction,
    id: &str,
    code: &str,
    redis_action_secondary: &RedisAction,
    redis_action_tertiary: RedisAction,
) -> Result<Option<RedisAccount>, AppError> {
    match result {
        Some(serialized) => {
            if is_temporarily_locked(state.clone(), redis_action_tertiary.as_ref(), id, 1).await? {
                return Ok(None);
            }

            let deserialized: RedisAccount = serde_json::from_str(serialized)?;

            let locked = match redis_action {
                RedisAction::Auth => {
                    check_locks(
                        state.clone(),
                        &deserialized.email,
                        deserialized.issued_timestamp.expect("auth account"),
                    )
                    .await?
                }
                _ => false,
            };

            if !locked && *redis_action != RedisAction::Update && code != deserialized.code {
                return Ok(None);
            }

            remove_id(
                state.clone(),
                redis_action.as_ref(),
                id,
                redis_action_secondary.as_ref(),
                &deserialized.email,
            )
            .await?;

            if locked {
                return Ok(None);
            }

            Ok(Some(deserialized))
        }
        None => Ok(None),
    }
}

async fn create_redis_account(
    state: Arc<AppState>,
    action: Action,
    email: &str,
    password: &str,
) -> Result<Option<RedisAccount>, AppError> {
    match get_user(state.clone(), email).await? {
        None => {
            if action == Action::Login {
                return Ok(None);
            }

            let password_owned = password.to_owned();

            let password_hash = spawn_blocking(move || hash_password(&password_owned)).await??;

            Ok(Some(RedisAccount {
                email: email.to_string(),
                action: action.clone(),
                code: generate_code().clone(),
                issued_timestamp: Some(Utc::now().timestamp_millis()),
                password_hash: Some(password_hash),
            }))
        }
        Some((hash, locked)) => {
            let plaintext = password.to_owned();

            let hash = hash.to_owned();

            if action == Action::Signup
                || locked
                || (action == Action::Login
                    && !spawn_blocking(move || verify_password(&plaintext, &hash)).await??)
            {
                return Ok(None);
            }

            Ok(Some(RedisAccount {
                email: email.to_string(),
                action: action.clone(),
                code: generate_code().clone(),
                issued_timestamp: Some(Utc::now().timestamp_millis()),
                password_hash: None,
            }))
        }
    }
}

async fn create_temporary_session(
    state: Arc<AppState>,
    result: &Option<String>,
    redis_account: &RedisAccount,
    redis_action: RedisAction,
    redis_action_secondary: RedisAction,
    ttl: i64,
) -> Result<HeaderMap, AppError> {
    if redis_action != RedisAction::Update {
        spawn_code_task(
            state.clone(),
            redis_account.email.clone(),
            redis_account.code.clone(),
        );
    }

    let serialized = match result {
        Some(result) => result,
        None => &serde_json::to_string(&redis_account)?,
    };

    let id = Uuid::new_v4().to_string();

    insert_id(
        state,
        redis_action.as_ref(),
        &id,
        serialized,
        redis_action_secondary.as_ref(),
        &redis_account.email,
        ttl.try_into().unwrap(),
    )
    .await?;

    Ok(generate_cookie(redis_action.as_ref(), &id, ttl))
}

async fn create_session(
    state: Arc<AppState>,
    redis_account: &RedisAccount,
    redis_action: RedisAction,
    redis_action_secondary: RedisAction,
    ttl: i64,
) -> Result<HeaderMap, AppError> {
    if redis_account.action == Action::Signup {
        insert_user(state.clone(), redis_account.clone()).await?;
    }

    let session_id = Uuid::new_v4().to_string();

    insert_session(
        state,
        redis_action.as_ref(),
        &session_id,
        redis_action_secondary.as_ref(),
        &redis_account.email,
    )
    .await?;

    Ok(generate_cookie(redis_action.as_ref(), &session_id, ttl))
}

async fn check_db_lock(state: Arc<AppState>, email: &str) -> Result<bool, AppError> {
    let locked = check_lock(state.clone(), email).await?;

    if locked.is_some() && locked.expect("is_some failed") {
        return Ok(true);
    }
    Ok(false)
}

async fn freeze_account(state: Arc<AppState>, email: &str) -> Result<(), AppError> {
    if check_db_lock(state.clone(), email).await? {
        return Ok(());
    }

    state
        .redis_connection_manager
        .clone()
        .set_ex(
            format!("{}:{}", RedisAction::LockedTime.as_ref(), &email),
            (Utc::now() + chronoDuration::milliseconds(500)).timestamp_millis(),
            900,
        )
        .await?;

    update_lock(state.clone(), email, true).await?;

    delete_all_sessions(
        state.clone(),
        RedisAction::Session.as_ref(),
        RedisAction::SessionStore.as_ref(),
        email,
    )
    .await?;

    Ok(())
}

async fn verify_token(
    state: Arc<AppState>,
    headers: HeaderMap,
) -> Result<Option<(Option<String>, RedisAction, RedisAction, String)>, AppError> {
    if let Some(id) = get_cookie(&headers, RedisAction::Forgot.as_ref()) {
        return Ok(Some((
            try_get(state.clone(), RedisAction::Forgot.as_ref(), &id).await?,
            RedisAction::Forgot,
            RedisAction::Recovering,
            id,
        )));
    }
    if let Some(id) = get_cookie(&headers, RedisAction::Auth.as_ref()) {
        return Ok(Some((
            try_get(state.clone(), RedisAction::Auth.as_ref(), &id).await?,
            RedisAction::Auth,
            RedisAction::Authenticating,
            id,
        )));
    }
    if let Some(id) = get_cookie(&headers, RedisAction::Update.as_ref()) {
        return Ok(Some((
            try_get(state.clone(), RedisAction::Update.as_ref(), &id).await?,
            RedisAction::Update,
            RedisAction::Updating,
            id,
        )));
    }
    Ok(None)
}

async fn unfreeze_account(
    state: Arc<AppState>,
    email: &str,
    password: &str,
) -> Result<(), AppError> {
    let password_owned = password.to_owned();

    unlock_account(
        state.clone(),
        email,
        &spawn_blocking(move || hash_password(&password_owned)).await??,
    )
    .await?;

    Ok(())
}
