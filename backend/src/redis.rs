use crate::{AppError, AppState};
use redis::{
    AsyncTypedCommands, Client,
    aio::{ConnectionManager, ConnectionManagerConfig},
};
use std::{
    env,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tracing::warn;

pub async fn init_redis() -> Result<ConnectionManager, AppError> {
    let redis_url = env::var("RUST_REDIS_URL").unwrap_or_else(|_| {
        warn!("Environment variable RUST_REDIS_URL not found, using default");
        "redis://redis:6379".to_string()
    });

    let client = Client::open(redis_url)?;

    let config = ConnectionManagerConfig::new()
        .set_number_of_retries(1)
        .set_connection_timeout(Duration::from_millis(100));

    let connection_manager = client.get_connection_manager_with_config(config).await?;
    Ok(connection_manager)
}

pub async fn insert_session(
    state: Arc<AppState>,
    session_id: &str,
    email: &str,
) -> Result<(), AppError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64();

    state
        .redis_connection_manager
        .clone()
        .set_ex(format!("session_id:{}", session_id), 1, 3600)
        .await?;

    state
        .redis_connection_manager
        .clone()
        .zadd(format!("sessions:{}", email), session_id, now)
        .await?;

    if state
        .redis_connection_manager
        .clone()
        .zcard(format!("sessions:{}", email))
        .await?
        > state.config.max_sessions.into()
    {
        state
            .redis_connection_manager
            .clone()
            .zremrangebyrank(format!("sessions:{}", email), 0, 0)
            .await?;
    }

    Ok(())
}

pub async fn insert_magic_link_token(
    state: Arc<AppState>,
    token: &str,
    serialized: &str,
    email: &str,
    ttl: u16,
) -> Result<(), AppError> {
    state
        .redis_connection_manager
        .clone()
        .set_ex(
            format!("magic_link_token:{}", token),
            serialized,
            ttl.into(),
        )
        .await?;

    state
        .redis_connection_manager
        .clone()
        .set_ex(format!("email:{}", email), 1, ttl.into())
        .await?;

    Ok(())
}

pub async fn remove_magic_link_token(
    state: Arc<AppState>,
    token: &str,
    email: &str,
) -> Result<(), AppError> {
    state
        .redis_connection_manager
        .clone()
        .del(format!("magic_link_token:{}", token))
        .await?;
    state
        .redis_connection_manager
        .clone()
        .del(format!("email:{}", email))
        .await?;

    Ok(())
}
