use crate::error::AppError;
use std::{env, fs::read_to_string};
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct Config {
    pub rust_port: u16,
    pub svelte_url: String,
    pub from_email: String,
    pub from_email_server: String,
    pub from_email_password: String,
    pub max_sessions: u8,
    pub api_token: String,
    pub auth_max_attempts: u8,
    pub auth_lock_duration_seconds: u16,
    pub verify_max_attempts: u8,
    pub verify_lock_duration_seconds: u16,
    pub temporary_session_duration_seconds: u16,
    pub session_duration_seconds: u16,
}

impl Config {
    pub fn load() -> Result<Self, AppError> {
        let rust_port = var("RUST_PORT")
            .inspect_err(|_| {
                info!("RUST_PORT not set, using default");
            })
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .map_err(|_| AppError::Config("Invalid RUST_PORT value".into()))?;

        let svelte_url = var("SVELTE_URL")
            .inspect_err(|_| {
                info!("SVELTE_URL not set, using default");
            })
            .unwrap_or_else(|_| "http://localhost:5173".into());

        let max_sessions = var("RUST_MAX_SESSIONS")
            .inspect_err(|_| {
                info!("RUST_MAX_SESSIONS not set, using default");
            })
            .unwrap_or_else(|_| "2".into())
            .parse()
            .map_err(|_| AppError::Config("Invalid RUST_MAX_SESSIONS value".into()))?;

        let auth_lock_duration_seconds = var("RUST_AUTH_LOCK_DURATION_SECS")
            .inspect_err(|_| {
                info!("RUST_AUTH_LOCK_DURATION_SECS not set, using default");
            })
            .unwrap_or_else(|_| "1800".into())
            .parse()
            .map_err(|_| AppError::Config("Invalid RUST_AUTH_LOCK_DURATION_SECS value".into()))?;

        let auth_max_attempts = var("PUBLIC_AUTH_MAX_ATTEMPTS")
            .inspect_err(|_| {
                info!("PUBLIC_AUTH_MAX_ATTEMPTS not set, using default");
            })
            .unwrap_or_else(|_| "15".into())
            .parse()
            .map_err(|_| AppError::Config("Invalid PUBLIC_AUTH_MAX_ATTEMPTS value".into()))?;

        let verify_lock_duration_seconds = var("RUST_VERIFY_LOCK_DURATION_SECS")
            .inspect_err(|_| {
                info!("RUST_VERIFY_LOCK_DURATION_SECS not set, using default");
            })
            .unwrap_or_else(|_| "600".into())
            .parse()
            .map_err(|_| AppError::Config("Invalid RUST_VERIFY_LOCK_DURATION_SECS value".into()))?;

        let verify_max_attempts = var("PUBLIC_VERIFY_MAX_ATTEMPTS")
            .inspect_err(|_| {
                info!("PUBLIC_VERIFY_MAX_ATTEMPTS not set, using default");
            })
            .unwrap_or_else(|_| "3".into())
            .parse()
            .map_err(|_| AppError::Config("Invalid PUBLIC_VERIFY_MAX_ATTEMPTS value".into()))?;

        let temporary_session_duration_seconds = var("PUBLIC_TEMP_SESSION_DURATION_SECS")
            .inspect_err(|_| {
                info!("PUBLIC_TEMP_SESSION_DURATION_SECS not set, using default");
            })
            .unwrap_or_else(|_| "600".into())
            .parse()
            .map_err(|_| {
                AppError::Config("Invalid PUBLIC_TEMP_SESSION_DURATION_SECS value".into())
            })?;

        let session_duration_seconds = var("RUST_SESSION_DURATION_SECS")
            .inspect_err(|_| {
                info!("RUST_SESSION_DURATION_SECS not set, using default");
            })
            .unwrap_or_else(|_| "3600".into())
            .parse()
            .map_err(|_| AppError::Config("Invalid RUST_SESSION_DURATION_SECS value".into()))?;

        let from_email = read_secret("RUST_FROM_EMAIL")
            .inspect_err(|_| {
                info!("RUST_FROM_EMAIL not set, using default");
            })
            .unwrap_or_else(|_| "WeAreInTroubleGoodnessGracious".into());

        let from_email_server = read_secret("RUST_FROM_EMAIL_SERVER")
            .inspect_err(|_| {
                info!("RUST_FROM_EMAIL_SERVER not set, using default");
            })
            .unwrap_or_else(|_| "ohdear".into());

        let from_email_password = read_secret("RUST_FROM_EMAIL_PASSWORD")
            .inspect_err(|_| {
                info!("RUST_FROM_EMAIL_PASSWORD not set, using default");
            })
            .unwrap_or_else(|_| "its so over".into());

        let api_token = read_secret("API_TOKEN")
            .inspect_err(|_| {
                info!("API_TOKEN not set, using default");
            })
            .unwrap_or_else(|_| "its so over".into());

        Ok(Self {
            rust_port,
            svelte_url,
            from_email,
            from_email_server,
            from_email_password,
            max_sessions,
            api_token,
            auth_max_attempts,
            auth_lock_duration_seconds,
            verify_max_attempts,
            verify_lock_duration_seconds,
            temporary_session_duration_seconds,
            session_duration_seconds,
        })
    }
}

fn var(key: &str) -> Result<String, AppError> {
    env::var(key).map_err(|e| {
        warn!("Environment variable {} not found, using default", key);
        AppError::Environment(e)
    })
}

fn read_secret(secret_name: &str) -> Result<String, AppError> {
    let path = format!("/run/secrets/{}", secret_name);
    read_to_string(&path)
        .map(|s| s.trim().to_string())
        .map_err(|e| {
            warn!("Failed to read {} from file: {}", secret_name, e);
            AppError::IO(e)
        })
}
