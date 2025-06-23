use super::{
    database::get_user,
    lock::check_locks,
    models::{Action, RedisAccount, RedisAction},
    twofactor::generate_code,
    verify::{hash_password, verify_password},
};
use crate::{AppError, AppState};
use chrono::Utc;
use redis::{
    AsyncTypedCommands, Client,
    aio::{ConnectionManager, ConnectionManagerConfig},
};
use std::{
    env,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::task::spawn_blocking;
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
    ttl: u16,
) -> Result<(), AppError> {
    state
        .redis_connection_manager
        .clone()
        .set_ex(format!("{}:{}", key, id), serialized, ttl.into())
        .await?;

    Ok(())
}

pub async fn remove_id(state: Arc<AppState>, key: &str, id: &str) -> Result<(), AppError> {
    state
        .redis_connection_manager
        .clone()
        .del(format!("{}:{}", key, id))
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

pub async fn get_redis_account(
    state: Arc<AppState>,
    result: &Option<String>,
    redis_action: &RedisAction,
    id: &str,
    code: &str,
    redis_action_secondary: RedisAction,
) -> Result<Option<RedisAccount>, AppError> {
    match result {
        Some(serialized) => {
            if is_temporarily_locked(state.clone(), redis_action_secondary.as_ref(), id, 1).await? {
                return Ok(None);
            }

            let deserialized: RedisAccount = serde_json::from_str(serialized)?;

            let locked = match redis_action {
                RedisAction::Auth => {
                    check_locks(
                        state.clone(),
                        &deserialized.email,
                        deserialized.issued_timestamp.expect("auth account"),
                    )
                    .await?
                }
                _ => false,
            };

            if !locked && *redis_action != RedisAction::Update && code != deserialized.code {
                return Ok(None);
            }

            remove_id(state.clone(), redis_action.as_ref(), id).await?;

            if locked {
                return Ok(None);
            }

            Ok(Some(deserialized))
        }
        None => Ok(None),
    }
}

pub async fn create_redis_account(
    state: Arc<AppState>,
    action: Action,
    email: &str,
    password: &str,
) -> Result<Option<RedisAccount>, AppError> {
    match get_user(state.clone(), email).await? {
        None => {
            if action == Action::Login {
                return Ok(None);
            }

            let password_owned = password.to_owned();

            let password_hash = spawn_blocking(move || hash_password(&password_owned)).await??;

            Ok(Some(RedisAccount {
                email: email.to_string(),
                action: action.clone(),
                code: generate_code().clone(),
                issued_timestamp: Some(Utc::now().timestamp_millis()),
                password_hash: Some(password_hash),
            }))
        }
        Some((hash, locked)) => {
            let plaintext = password.to_owned();

            let hash = hash.to_owned();

            if action == Action::Signup
                || locked
                || (action == Action::Login
                    && !spawn_blocking(move || verify_password(&plaintext, &hash)).await??)
            {
                return Ok(None);
            }

            Ok(Some(RedisAccount {
                email: email.to_string(),
                action: action.clone(),
                code: generate_code().clone(),
                issued_timestamp: Some(Utc::now().timestamp_millis()),
                password_hash: None,
            }))
        }
    }
}
