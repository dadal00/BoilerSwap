use crate::{error::AppError, state::AppState, state::DatabaseQueries};
use scylla::{
    client::{session::Session, session_builder::SessionBuilder},
    response::{PagingState, query_result::FirstRowError::RowsEmpty},
};
use std::{env, sync::Arc};
use tracing::warn;
use uuid::Uuid;

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
            "CREATE TABLE IF NOT EXISTS boiler_swap.user_by_email (
                email text,
                user_id uuid,
                PRIMARY KEY(email)
            )",
            &[],
        )
        .await?;

    database_session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS boiler_swap.user_info (
                user_id uuid,
                password_hash text,
                PRIMARY KEY(user_id)
            )",
            &[],
        )
        .await?;

    database_session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS boiler_swap.product_by_user (
                user_id uuid,
                product_id uuid,
                PRIMARY KEY(user_id, product_id)
            )",
            &[],
        )
        .await?;

    database_session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS boiler_swap.product_info (
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

    database_session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS boiler_swap.user_sessions (
                session_id uuid,
                user_id uuid,
                ip_address inet,
                user_agent text,
                PRIMARY KEY (session_id)
            ) WITH default_time_to_live = 3600",
            &[],
        )
        .await?;

    let database_queries = DatabaseQueries {
        get_user_id: database_session
            .prepare("SELECT user_id FROM boiler_swap.user_by_email WHERE email = ?")
            .await?,
    };

    Ok((Arc::new(database_session), database_queries))
}

pub async fn get_user(state: Arc<AppState>, email: &str) -> Result<Option<Uuid>, AppError> {
    let fallback_page_state = PagingState::start();
    let (returned_rows, _) = state
        .database_session
        .execute_single_page(
            &state.database_queries.get_user_id,
            (email,),
            fallback_page_state,
        )
        .await?;

    match returned_rows.into_rows_result()?.first_row::<(Uuid,)>() {
        Ok((user_id,)) => Ok(Some(user_id)),
        Err(RowsEmpty) => Ok(None),
        Err(e) => Err(e.into()),
    }
}
