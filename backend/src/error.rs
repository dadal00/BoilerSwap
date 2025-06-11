use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{env::VarError, io::Error as IOError};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Environment error: {0}")]
    Environment(#[from] VarError),

    #[error("IO error: {0}")]
    IO(#[from] IOError),

    #[error("Configuration error: {0}")]
    Config(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = {
            error!("Server error: {}", self);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
        };

        (status, message).into_response()
    }
}
