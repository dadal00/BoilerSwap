use super::{models::RedisAction, redis::try_get, sessions::get_cookie};
use crate::{AppError, AppState};
use argon2::{
    Algorithm::Argon2id, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
    Version::V0x13, password_hash::SaltString,
};
use axum::http::header::{AUTHORIZATION, HeaderMap};
use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use regex::Regex;
use std::sync::Arc;
use tracing::warn;

pub static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.+@purdue\.edu$").unwrap());

pub async fn verify_token(
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

pub fn validate_api_token(headers: HeaderMap, real_api_token: &str) -> bool {
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

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|e| {
        warn!("Failed to parse password hash: {}", e);
        AppError::Config(e.to_string())
    })?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn validate_account(email: &str, password: &str) -> Result<(), &'static str> {
    validate_email(email)?;

    validate_password(password)?;

    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.len() > 100 {
        return Err("Too many chars");
    }

    if password.is_empty() {
        return Err("Password cannot be empty");
    }

    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), &'static str> {
    if email.len() > 100 {
        return Err("Too many chars");
    }

    if !EMAIL_REGEX.is_match(email) {
        return Err("Email must be a Purdue address");
    }

    Ok(())
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
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
