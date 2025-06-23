use super::{
    database::{check_lock, unlock_account, update_lock},
    models::RedisAction,
    redis::{delete_all_sessions, try_get},
    verify::hash_password,
};
use crate::{AppError, AppState};
use chrono::{Duration as chronoDuration, Utc};
use redis::AsyncTypedCommands;
use std::sync::Arc;
use tokio::task::spawn_blocking;

pub async fn check_db_lock(state: Arc<AppState>, email: &str) -> Result<bool, AppError> {
    let locked = check_lock(state.clone(), email).await?;

    if locked.is_some() && locked.expect("is_some failed") {
        return Ok(true);
    }
    Ok(false)
}

pub async fn freeze_account(state: Arc<AppState>, email: &str) -> Result<(), AppError> {
    if check_db_lock(state.clone(), email).await? {
        return Ok(());
    }

    state
        .redis_connection_manager
        .clone()
        .set_ex(
            format!("{}:{}", RedisAction::LockedTime.as_ref(), &email),
            (Utc::now() + chronoDuration::milliseconds(500)).timestamp_millis(),
            900,
        )
        .await?;

    update_lock(state.clone(), email, true).await?;

    delete_all_sessions(
        state.clone(),
        RedisAction::Session.as_ref(),
        RedisAction::SessionStore.as_ref(),
        email,
    )
    .await?;

    Ok(())
}

pub async fn unfreeze_account(
    state: Arc<AppState>,
    email: &str,
    password: &str,
) -> Result<(), AppError> {
    let password_owned = password.to_owned();

    unlock_account(
        state.clone(),
        email,
        &spawn_blocking(move || hash_password(&password_owned)).await??,
    )
    .await?;

    Ok(())
}

pub async fn check_locks(
    state: Arc<AppState>,
    email: &str,
    issued_timestamp: i64,
) -> Result<bool, AppError> {
    if check_db_lock(state.clone(), email).await? {
        return Ok(true);
    }

    let locked_timestamp = try_get(state.clone(), RedisAction::LockedTime.as_ref(), email).await?;

    if locked_timestamp.is_some()
        && issued_timestamp
            < locked_timestamp
                .expect("is_some failed")
                .parse::<i64>()
                .unwrap_or(i64::MAX)
    {
        return Ok(true);
    }

    Ok(false)
}
