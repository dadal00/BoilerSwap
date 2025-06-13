use crate::{AppError, database::get_user, state::AppState};
use axum::{
    Json,
    extract::{ConnectInfo, State},
    http::{StatusCode, header::HeaderMap},
    response::IntoResponse,
};
use once_cell::sync::Lazy;
use rand::{Rng, SeedableRng, rng, rngs::StdRng};
use regex::Regex;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tracing::info;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Account {
    email: String,
    password: String,
}

#[derive(Clone)]
pub struct VerificationInfo {
    code: String,
    user_id: Uuid,
    client_hash: String,
    attempts: u8,
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

    let user_id_future = get_user(state.clone(), &payload.email);

    let client_signature = get_client_signature(&headers, address.ip());
    let client_hash = hash(client_signature.clone(), state.config.hash_salt.clone());

    let mut rng = StdRng::from_rng(&mut rng());
    let code = rng.random_range(0..1_000_000);

    let authentication_id = Uuid::new_v4();

    let mut user_id = user_id_future.await?;

    if user_id.is_none() {
        user_id = Some(Uuid::new_v4());
    }

    let verification_entry = VerificationInfo {
        code: format!("{:06}", code),
        user_id: user_id.expect("UserID not found"),
        client_hash,
        attempts: 0,
    };

    state
        .verification_map
        .insert(authentication_id, verification_entry)
        .await;

    Ok((StatusCode::OK, authentication_id.to_string()).into_response())
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

fn get_client_signature(headers: &HeaderMap, direct_ip: IpAddr) -> String {
    let ip = headers
        .get("cf-connecting-ip")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| {
            headers
                .get("x-forwarded-for")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.split(',').next().map(|s| s.trim().to_string()))
        })
        .unwrap_or_else(|| direct_ip.to_string());

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown-user-agent");

    format!("{} {}", ip, user_agent)
}

fn hash(to_hash: String, hash_salt: String) -> String {
    let mut hasher = Sha256::new();

    hasher.update(hash_salt.as_bytes());
    hasher.update(to_hash.as_bytes());

    format!("{:x}", hasher.finalize())
}
