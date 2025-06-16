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

        Ok(Self {
            rust_port,
            svelte_url,
            from_email,
            from_email_server,
            from_email_password,
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
