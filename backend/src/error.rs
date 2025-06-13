use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use prometheus::Error as prometheusError;
use scylla::errors::{ExecutionError, NewSessionError};
use std::{env::VarError, io::Error as IOError, string::FromUtf8Error};
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

    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("Prometheus error: {0}")]
    Prometheus(#[from] prometheusError),

    #[error("ScyllaDB error: {0}")]
    ScyllaDBInit(#[from] NewSessionError),

    #[error("ScyllaDB error: {0}")]
    ScyllaDBExecute(#[from] ExecutionError),
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
