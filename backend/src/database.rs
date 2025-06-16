use crate::{error::AppError, state::AppState, state::DatabaseQueries};
use scylla::{
    client::{session::Session, session_builder::SessionBuilder},
    response::{PagingState, query_result::FirstRowError::RowsEmpty},
};
use std::{env, sync::Arc};
use tracing::warn;

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
            "CREATE TABLE IF NOT EXISTS boiler_swap.user_info (
                email text,
                password_hash text,
                PRIMARY KEY(email)
            )",
            &[],
        )
        .await?;

    database_session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS boiler_swap.product_by_user (
                email text,
                product_id uuid,
                PRIMARY KEY(email, product_id)
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

    let database_queries = DatabaseQueries {
        get_user_info: database_session
            .prepare("SELECT password_hash FROM boiler_swap.user_info WHERE email = ?")
            .await?,
    };

    Ok((Arc::new(database_session), database_queries))
}

pub async fn get_user(state: Arc<AppState>, email: &str) -> Result<Option<String>, AppError> {
    let fallback_page_state = PagingState::start();
    let (returned_rows, _) = state
        .database_session
        .execute_single_page(
            &state.database_queries.get_user_info,
            (email,),
            fallback_page_state,
        )
        .await?;

    match returned_rows.into_rows_result()?.first_row::<(String,)>() {
        Ok((password_hash,)) => Ok(Some(password_hash)),
        Err(RowsEmpty) => Ok(None),
        Err(e) => Err(e.into()),
    }
}
