use crate::{
    AppError,
    database::{get_user, insert_user},
    redis::{insert_magic_link_token, insert_session, remove_magic_link_token},
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
        StatusCode,
        header::{AUTHORIZATION, HeaderMap},
    },
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use once_cell::sync::Lazy;
use rand::{RngCore, rngs::OsRng};
use redis::AsyncTypedCommands;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::task::spawn_blocking;
use tracing::{debug, info, warn};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(Clone)]
pub enum Action {
    Login,
    Signup,
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
}

#[derive(Deserialize)]
pub struct Token {
    token: String,
}

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.+@purdue\.edu$").unwrap());

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

pub async fn verify_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Token>,
) -> Result<impl IntoResponse, AppError> {
    let redis_account = match state
        .redis_connection_manager
        .clone()
        .get(format!("magic_link_token:{}", payload.token))
        .await?
    {
        Some(serialized) => {
            let deserialized: RedisAccount = serde_json::from_str(&serialized)?;

            remove_magic_link_token(state.clone(), &payload.token, &deserialized.email).await?;

            deserialized
        }
        None => {
            return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
        }
    };

    if redis_account.action == Action::Signup && redis_account.password_hash.is_some() {
        insert_user(state.clone(), redis_account.clone()).await?;
    }

    let session_id = Uuid::new_v4().to_string();
    insert_session(state, &session_id, &redis_account.email).await?;

    Ok((StatusCode::OK, session_id).into_response())
}

pub async fn authenticate_handler(
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Account>,
) -> Result<impl IntoResponse, AppError> {
    if state
        .redis_connection_manager
        .clone()
        .get(format!("email:{}", &payload.email))
        .await?
        .is_some()
    {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
    }

    if let Err(e) = validate_account(&payload.email, &payload.password) {
        return Ok((StatusCode::BAD_REQUEST, e).into_response());
    }

    let redis_account = match get_user(state.clone(), &payload.email).await? {
        None => {
            if payload.action == Action::Login {
                return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
            }

            let password_hash = spawn_blocking(move || hash_password(&payload.password)).await??;
            RedisAccount {
                email: payload.email.clone(),
                action: payload.action.clone(),
                password_hash: Some(password_hash),
            }
        }
        Some(hash) => {
            let plaintext = payload.password.to_owned();
            let hash = hash.to_owned();
            if payload.action == Action::Login
                && !spawn_blocking(move || verify_password(&plaintext, &hash)).await??
            {
                return Ok((StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response());
            }

            RedisAccount {
                email: payload.email.clone(),
                action: payload.action.clone(),
                password_hash: None,
            }
        }
    };

    let magic_link_token = generate_magic_link_token();
    spawn_magic_link_task(
        state.clone(),
        payload.email.clone(),
        payload.action.clone(),
        magic_link_token.clone(),
    );

    let serialized = serde_json::to_string(&redis_account)?;
    insert_magic_link_token(state, &magic_link_token, &serialized, &payload.email, 600).await?;

    Ok((StatusCode::OK).into_response())
}

fn validate_account(email: &str, password: &str) -> Result<(), &'static str> {
    if !EMAIL_REGEX.is_match(email) {
        return Err("Email must be a Purdue address");
    }

    if password.is_empty() {
        return Err("Password cannot be empty");
    }

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

fn generate_magic_link_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

async fn send_magic_link_email(
    state: Arc<AppState>,
    user_email: &str,
    user_action: &Action,
    magic_link_token: &str,
) -> Result<(), AppError> {
    let email = Message::builder()
        .from(format!("BoilerSwap <{}>", state.config.from_email).parse()?)
        .to(user_email.parse()?)
        .subject("BoilerSwap Link")
        .body(format!(
            "Click this link to login:\n\n{}/auth/verify?token={}",
            state.config.svelte_url, magic_link_token
        ))?;

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

fn spawn_magic_link_task(state: Arc<AppState>, email: String, action: Action, token: String) {
    tokio::spawn(async move {
        if let Err(error) = send_magic_link_email(state, &email, &action, &token).await {
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
    if let Some(api_token) = CookieJar::from_headers(&headers).get("api_token") {
        if api_token
            .to_string()
            .split_once('=')
            .map(|x| x.1)
            .unwrap_or("")
            == real_api_token
        {
            return true;
        }
    }
    false
}
