use crate::{AppError, database::get_user, state::AppState};
use argon2::{
    Algorithm::Argon2id, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
    Version::V0x13, password_hash::SaltString,
};
use axum::{
    Json,
    extract::{ConnectInfo, State},
    http::{StatusCode, header::HeaderMap},
    response::IntoResponse,
};
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

#[derive(Serialize, Deserialize)]
pub struct RedisAccount {
    email: String,
    action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_hash: Option<String>,
}

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.+@purdue\.edu$").unwrap());

pub async fn default_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    info!("Received!");
    "Hello from Axum!"
}

pub async fn authenticate_handler(
    headers: HeaderMap,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Account>,
) -> Result<impl IntoResponse, AppError> {
    if let Err(e) = validate_account(&payload.email, &payload.password) {
        return Ok((StatusCode::BAD_REQUEST, e).into_response());
    }

    /*
    password is none
    - login -> invalid
    - signup -> valid proceed (magic link)
    password is not none
    - login -> check password
        - not match -> invalid
        - match -> valid proceed (magic link)
    - signup -> valid proceed (email check, magic link)
    */

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

    let state_clone = state.clone();
    let user_email = payload.email.clone();
    let user_action = payload.action.clone();
    let token = magic_link_token.clone();
    tokio::spawn(async move {
        if let Err(error) =
            send_magic_link_email(state_clone, &user_email, &user_action, &token).await
        {
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

    let serialized = serde_json::to_string(&redis_account)?;
    let key = format!("magic_link_token:{}", magic_link_token);
    state
        .redis_connection_manager
        .clone()
        .set_ex(&key, serialized, 600)
        .await?;

    Ok((StatusCode::OK, "authentication_id".to_string()).into_response())
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
