use axum::response::IntoResponse;
use tracing::info;

pub async fn default_handler() -> impl IntoResponse {
    info!("Received!");
    "Hello from Axum!"
}
