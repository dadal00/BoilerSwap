use crate::{
    api::{authenticate_handler, default_handler},
    error::AppError,
    metrics::metrics_handler,
    signals::shutdown_signal,
    state::AppState,
};
use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
    routing::{get, post},
};
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

mod api;
mod config;
mod database;
mod error;
mod metrics;
mod redis;
mod signals;
mod state;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    fmt()
        .with_env_filter(
            EnvFilter::from_default_env(), // backend (target) = info (logging level)
        )
        .init();

    info!("Starting server...");

    let state = AppState::new().await?;

    info!("Server configuration");
    info!("rust_port = {}", state.config.rust_port);
    info!("svelte_url = {}", state.config.svelte_url);

    let origin_state = state.clone();
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(move |origin, _req| {
            origin.as_bytes() == origin_state.config.svelte_url.as_bytes()
        }))
        .allow_methods([Method::GET, Method::OPTIONS, Method::POST])
        .allow_headers([CONTENT_TYPE])
        .max_age(Duration::from_secs(60 * 60));

    let app = Router::new()
        .route("/api/authenticate", post(authenticate_handler))
        .route("/api/verify", get(default_handler))
        .route("/api/post-item", get(default_handler))
        .route("/api/get-items", get(default_handler))
        .route("/metrics", get(metrics_handler))
        .layer(cors)
        .with_state(state.clone())
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = format!("0.0.0.0:{}", state.config.rust_port);
    info!("Binding to {}", addr);

    let listener = TcpListener::bind(&addr).await?;
    info!("Server running on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
