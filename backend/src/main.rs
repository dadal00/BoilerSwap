use crate::{
    api::default_handler,
    config::Config,
    error::AppError,
    metrics::{Metrics, metrics_handler},
    signals::shutdown_signal,
    state::AppState,
};
use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
    routing::get,
};
use std::{sync::Arc, time::Duration};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

mod api;
mod config;
mod error;
mod metrics;
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

    let state = Arc::new(AppState {
        config: Config::load()?,
        metrics: Metrics::default(),
    });

    info!("Server configuration");
    info!("rust_port = {}", state.config.rust_port);
    info!("svelte_url = {}", state.config.svelte_url);

    let origin_state = state.clone();
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(move |origin, _req| {
            origin.as_bytes() == origin_state.config.svelte_url.as_bytes()
        }))
        .allow_methods([Method::GET, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE])
        .max_age(Duration::from_secs(60 * 60));

    let app = Router::new()
        .route("/login", get(default_handler))
        .route("/metrics", get(metrics_handler))
        .layer(cors)
        .with_state(state.clone());

    let addr = format!("0.0.0.0:{}", state.config.rust_port);
    info!("Binding to {}", addr);

    let listener = TcpListener::bind(&addr).await?;
    info!("Server running on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
