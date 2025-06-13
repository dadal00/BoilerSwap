use crate::error::AppError;
use scylla::client::{session::Session, session_builder::SessionBuilder};
use std::{env, sync::Arc};
use tracing::warn;

pub async fn init_db() -> Result<Arc<Session>, AppError> {
    let db_uri = env::var("RUST_DB_URI").unwrap_or_else(|_| {
        warn!("Environment variable RUST_DB_URL not found, using default");
        "scylladb:9042".to_string()
    });

    let db_session: Session = SessionBuilder::new().known_node(db_uri).build().await?;

    db_session
        .query_unpaged(
            "CREATE KEYSPACE IF NOT EXISTS boiler_swap WITH REPLICATION = {'class': 'SimpleStrategy', 'replication_factor': 1}",
            &[],
        )
        .await?;

    db_session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS boiler_swap.user_by_email (
                email text,
                user_id uuid,
                PRIMARY KEY(email)
            )",
            &[],
        )
        .await?;

    db_session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS boiler_swap.user_info (
                user_id uuid,
                password_hash text,
                PRIMARY KEY(user_id)
            )",
            &[],
        )
        .await?;

    db_session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS boiler_swap.product_by_user (
                user_id uuid,
                product_id uuid,
                PRIMARY KEY(user_id, product_id)
            )",
            &[],
        )
        .await?;

    db_session
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

    Ok(Arc::new(db_session))
}
