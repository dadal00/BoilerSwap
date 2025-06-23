use super::models::RedisAccount;
use crate::{error::AppError, state::AppState};
use scylla::{
    client::{session::Session, session_builder::SessionBuilder},
    response::{PagingState, query_result::FirstRowError::RowsEmpty},
    statement::prepared::PreparedStatement,
};
use std::{env, sync::Arc};
use tracing::warn;

pub struct DatabaseQueries {
    pub get_user: PreparedStatement,
    pub insert_user: PreparedStatement,
    pub check_lock: PreparedStatement,
    pub update_lock: PreparedStatement,
    pub unlock_account: PreparedStatement,
}

pub async fn init_database() -> Result<(Arc<Session>, DatabaseQueries), AppError> {
    let database_uri = env::var("RUST_DB_URI").unwrap_or_else(|_| {
        warn!("Environment variable RUST_DB_URL not found, using default");
        "scylladb:9042".to_string()
    });

    let database_session: Session = SessionBuilder::new()
        .known_node(database_uri)
        .build()
        .await?;

    database_session
        .query_unpaged(
            "CREATE KEYSPACE IF NOT EXISTS boiler_swap WITH REPLICATION = {'class': 'SimpleStrategy', 'replication_factor': 1}",
            &[],
        )
        .await?;

    database_session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS boiler_swap.users (
                email text,
                password_hash text,
                locked boolean,
                PRIMARY KEY(email)
            )",
            &[],
        )
        .await?;

    database_session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS boiler_swap.products (
                product_id uuid,
                item_type tinyint,
                title text,
                condition tinyint,
                location tinyint,
                description text,
                PRIMARY KEY(product_id)
            ) WITH cdc = {'enabled': true}",
            &[],
        )
        .await?;

    let database_queries = DatabaseQueries {
        get_user: database_session
            .prepare("SELECT password_hash, locked FROM boiler_swap.users WHERE email = ?")
            .await?,
        insert_user: database_session
            .prepare("INSERT INTO boiler_swap.users (email, password_hash, locked) VALUES (?, ?, ?) USING TTL 126144000")
            .await?,
        check_lock: database_session
            .prepare("SELECT locked FROM boiler_swap.users WHERE email = ?")
            .await?,
        update_lock: database_session
            .prepare("UPDATE boiler_swap.users SET locked = ? WHERE email = ?")
            .await?,
        unlock_account: database_session
            .prepare("UPDATE boiler_swap.users SET locked = false, password_hash = ? WHERE email = ?")
            .await?,
    };

    Ok((Arc::new(database_session), database_queries))
}

pub async fn get_user(
    state: Arc<AppState>,
    email: &str,
) -> Result<Option<(String, bool)>, AppError> {
    let fallback_page_state = PagingState::start();
    let (returned_rows, _) = state
        .database_session
        .execute_single_page(
            &state.database_queries.get_user,
            (email,),
            fallback_page_state,
        )
        .await?;

    match returned_rows
        .into_rows_result()?
        .first_row::<(String, bool)>()
    {
        Ok((password_hash, locked)) => Ok(Some((password_hash, locked))),
        Err(RowsEmpty) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub async fn insert_user(state: Arc<AppState>, account: RedisAccount) -> Result<(), AppError> {
    let fallback_page_state = PagingState::start();
    state
        .database_session
        .execute_single_page(
            &state.database_queries.insert_user,
            (account.email, account.password_hash, false),
            fallback_page_state,
        )
        .await?;

    Ok(())
}

pub async fn check_lock(state: Arc<AppState>, email: &str) -> Result<Option<bool>, AppError> {
    let fallback_page_state = PagingState::start();
    let (returned_rows, _) = state
        .database_session
        .execute_single_page(
            &state.database_queries.check_lock,
            (email,),
            fallback_page_state,
        )
        .await?;

    match returned_rows.into_rows_result()?.first_row::<(bool,)>() {
        Ok((locked,)) => Ok(Some(locked)),
        Err(RowsEmpty) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub async fn update_lock(state: Arc<AppState>, email: &str, lock: bool) -> Result<(), AppError> {
    let fallback_page_state = PagingState::start();
    state
        .database_session
        .execute_single_page(
            &state.database_queries.update_lock,
            (lock, email),
            fallback_page_state,
        )
        .await?;

    Ok(())
}

pub async fn unlock_account(
    state: Arc<AppState>,
    email: &str,
    password_hash: &str,
) -> Result<(), AppError> {
    let fallback_page_state = PagingState::start();
    state
        .database_session
        .execute_single_page(
            &state.database_queries.unlock_account,
            (password_hash, email),
            fallback_page_state,
        )
        .await?;

    Ok(())
}
