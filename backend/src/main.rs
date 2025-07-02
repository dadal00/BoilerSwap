use crate::{
    api::{
        database::start_cdc,
        handlers::{
            api_token_check, authenticate_handler, default_handler, delete_handler, forgot_handler,
            post_item_handler, verify_handler,
        },
        schema::{KEYSPACE, columns::items, tables},
    },
    error::AppError,
    metrics::metrics_handler,
    signals::shutdown_signal,
    state::AppState,
};
use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
    middleware,
    routing::{delete, get, post},
};
use std::{net::SocketAddr, time::Duration};
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

    let (state, meili_reindex_future) = AppState::new().await?;

    info!("Server configuration");
    info!("rust_port = {}", state.config.rust_port);
    info!("svelte_url = {}", state.config.svelte_url);

    let origin_state = state.clone();
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(move |origin, _req| {
            origin.as_bytes() == origin_state.config.svelte_url.as_bytes()
        }))
        .allow_methods([Method::GET, Method::OPTIONS, Method::POST, Method::DELETE])
        .allow_headers([CONTENT_TYPE])
        .max_age(Duration::from_secs(60 * 60));

    let app = Router::new()
        .route("/api/authenticate", post(authenticate_handler))
        .route("/api/verify", post(verify_handler))
        .route("/api/delete", delete(delete_handler))
        .route("/api/forgot", post(forgot_handler))
        .route("/api/post-item", post(post_item_handler))
        .route("/api/get-items", get(default_handler))
        .route("/metrics", get(metrics_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            api_token_check,
        ))
        .layer(cors)
        .with_state(state.clone())
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = format!("0.0.0.0:{}", state.config.rust_port);
    info!("Binding to {}", addr);

    let listener = TcpListener::bind(&addr).await?;

    meili_reindex_future.await??;

    let (mut cdc_reader, cdc_future) =
        start_cdc(state.clone(), KEYSPACE, tables::ITEMS, items::ITEM_ID).await?;

    info!("Server running on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    cdc_reader.stop();

    Ok(cdc_future.await?)
}
