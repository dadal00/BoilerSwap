use crate::error::AppError;
use std::{env, fs::read_to_string};
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct Config {
    pub rust_port: u16,
    pub svelte_url: String,
    pub hash_salt: String,
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

        let hash_salt = read_hash_salt()
            .inspect_err(|_| {
                info!("RUST_HASH_SALT not set, using default");
            })
            .unwrap_or_else(|_| "WeAreInTroubleGoodnessGracious".into());

        Ok(Self {
            rust_port,
            svelte_url,
            hash_salt,
        })
    }
}

fn var(key: &str) -> Result<String, AppError> {
    env::var(key).map_err(|e| {
        warn!("Environment variable {} not found, using default", key);
        AppError::Environment(e)
    })
}

fn read_hash_salt() -> Result<String, AppError> {
    read_to_string("/run/secrets/RUST_HASH_SALT")
        .map(|s| s.trim().to_string())
        .map_err(|e| {
            warn!("Failed to read RUST_HASH_SALT from file: {}", e);
            AppError::IO(e)
        })
}
