use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lettre::{
    address::AddressError, error::Error as lettreGeneralError,
    transport::smtp::Error as lettreTransportError,
};
use prometheus::Error as prometheusError;
use redis::RedisError;
use scylla::errors::{
    ExecutionError, FirstRowError, IntoRowsResultError, NewSessionError, PrepareError,
};
use serde_json::Error as serdeJsonError;
use std::{env::VarError, io::Error as IOError, string::FromUtf8Error};
use thiserror::Error;
use tokio::task::JoinError;
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

    #[error("ScyllaDB new session error: {0}")]
    ScyllaInit(#[from] NewSessionError),

    #[error("ScyllaDB execution error: {0}")]
    ScyllaExecute(#[from] ExecutionError),

    #[error("ScyllaDB prepare error: {0}")]
    ScyllaPrepare(#[from] PrepareError),

    #[error("ScyllaDB rows result error: {0}")]
    ScyllaRowsResult(#[from] IntoRowsResultError),

    #[error("ScyllaDB first row error: {0}")]
    ScyllaFirstRow(#[from] FirstRowError),

    #[error("Tokio join error: {0}")]
    TokioJoin(#[from] JoinError),

    #[error("Lettre transport error: {0}")]
    LettreTransport(#[from] lettreTransportError),

    #[error("Lettre address error: {0}")]
    LettreAddress(#[from] AddressError),

    #[error("Lettre general error: {0}")]
    LettreGeneral(#[from] lettreGeneralError),

    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),

    #[error("SerdeJson error: {0}")]
    ToJson(#[from] serdeJsonError),
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
