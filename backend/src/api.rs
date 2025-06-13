use crate::state::AppState;
use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;
use tracing::info;

pub async fn default_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    info!("Received!");
    "Hello from Axum!"
}
