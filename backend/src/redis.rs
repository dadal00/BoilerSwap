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

pub async fn delete_all_sessions(
    state: Arc<AppState>,
    key: &str,
    key_secondary: &str,
    email: &str,
) -> Result<(), AppError> {
    let mut pipe = redis::pipe();

    for session_id in state
        .redis_connection_manager
        .clone()
        .zrange(format!("{}:{}", key_secondary, email), 0, -1)
        .await?
    {
        pipe.del(format!("{}:{}", key, session_id)).ignore();
    }

    pipe.del(format!("{}:{}", key_secondary, email)).ignore();

    pipe.query_async::<()>(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(())
}

pub async fn insert_session(
    state: Arc<AppState>,
    key: &str,
    session_id: &str,
    key_secondary: &str,
    email: &str,
) -> Result<(), AppError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64();

    state
        .redis_connection_manager
        .clone()
        .set_ex(format!("{}:{}", key, session_id), 1, 3600)
        .await?;

    state
        .redis_connection_manager
        .clone()
        .zadd(format!("{}:{}", key_secondary, email), session_id, now)
        .await?;

    if state
        .redis_connection_manager
        .clone()
        .zcard(format!("{}:{}", key_secondary, email))
        .await?
        > state.config.max_sessions.into()
    {
        state
            .redis_connection_manager
            .clone()
            .zremrangebyrank(format!("{}:{}", key_secondary, email), 0, 0)
            .await?;
    }

    Ok(())
}

pub async fn insert_id(
    state: Arc<AppState>,
    key: &str,
    id: &str,
    serialized: &str,
    key_secondary: &str,
    email: &str,
    ttl: u16,
) -> Result<(), AppError> {
    state
        .redis_connection_manager
        .clone()
        .set_ex(format!("{}:{}", key, id), serialized, ttl.into())
        .await?;

    state
        .redis_connection_manager
        .clone()
        .pset_ex(format!("{}:{}", key_secondary, email), 1, 500)
        .await?;

    Ok(())
}

pub async fn remove_id(
    state: Arc<AppState>,
    key: &str,
    id: &str,
    key_secondary: &str,
    email: &str,
) -> Result<(), AppError> {
    state
        .redis_connection_manager
        .clone()
        .del(format!("{}:{}", key, id))
        .await?;
    state
        .redis_connection_manager
        .clone()
        .del(format!("{}:{}", key_secondary, email))
        .await?;

    Ok(())
}

pub async fn is_temporarily_locked(
    state: Arc<AppState>,
    key: &str,
    id: &str,
    ttl: i64,
) -> Result<bool, AppError> {
    let result: Option<String> = redis::cmd("SET")
        .arg(format!("{}:{}", key, id))
        .arg("1")
        .arg("NX")
        .arg("EX")
        .arg(ttl)
        .query_async(&mut state.redis_connection_manager.clone())
        .await?;

    Ok(result.is_none())
}

pub async fn try_get(
    state: Arc<AppState>,
    key: &str,
    email: &str,
) -> Result<Option<String>, AppError> {
    Ok(state
        .redis_connection_manager
        .clone()
        .get(format!("{}:{}", key, email))
        .await?)
}
