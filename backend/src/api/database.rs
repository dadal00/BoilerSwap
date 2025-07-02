use super::{
    models::{Condition, Item, ItemPayload, ItemRow, ItemType, Location, RedisAccount},
    utilities::get_seconds_until,
};
use crate::{error::AppError, state::AppState};
use chrono::Weekday;
use scylla::{
    client::{session::Session, session_builder::SessionBuilder},
    response::{PagingState, query_result::FirstRowError::RowsEmpty},
    statement::{prepared::PreparedStatement, unprepared::Statement},
};
use std::{env, sync::Arc};
use tracing::warn;
use uuid::Uuid;

#[derive(Clone)]
pub struct DatabaseQueries {
    pub get_user: PreparedStatement,
    pub insert_user: PreparedStatement,
    pub check_lock: PreparedStatement,
    pub update_lock: PreparedStatement,
    pub unlock_account: PreparedStatement,
    pub insert_item: PreparedStatement,
    pub get_items: PreparedStatement,
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
            "CREATE TABLE IF NOT EXISTS boiler_swap.items (
                item_id uuid,
                item_type tinyint,
                title text,
                condition tinyint,
                location tinyint,
                description text,
                PRIMARY KEY(item_id)
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
        insert_item: database_session
            .prepare("INSERT INTO boiler_swap.items (item_id, item_type, title, condition, location, description) VALUES (?, ?, ?, ?, ?, ?) USING TTL ?")
            .await?,
        get_items: database_session
            .prepare(Statement::new("SELECT * FROM boiler_swap.items")
            .with_page_size(100),)
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

pub async fn insert_item(state: Arc<AppState>, item: ItemPayload) -> Result<(), AppError> {
    let fallback_page_state = PagingState::start();
    state
        .database_session
        .execute_single_page(
            &state.database_queries.insert_item,
            (
                Uuid::new_v4(),
                item.item_type as i8,
                item.title,
                item.condition as i8,
                item.location as i8,
                item.description,
                get_seconds_until(Weekday::Thu),
            ),
            fallback_page_state,
        )
        .await?;

    Ok(())
}

pub fn convert_db_items(row_vec: &Vec<ItemRow>) -> Vec<Item> {
    row_vec
        .iter()
        .map(
            |(id, item_type_i8, title, condition_i8, location_i8, description)| Item {
                item_id: *id,
                item_type: ItemType::try_from(item_type_i8.checked_abs().unwrap_or(0) as u8)
                    .unwrap_or(ItemType::Other)
                    .as_ref()
                    .to_string(),
                title: title.to_string(),
                condition: Condition::try_from(condition_i8.checked_abs().unwrap_or(0) as u8)
                    .unwrap_or(Condition::Fair)
                    .as_ref()
                    .to_string(),
                location: Location::try_from(location_i8.checked_abs().unwrap_or(0) as u8)
                    .unwrap_or(Location::CaryQuadEast)
                    .as_ref()
                    .to_string(),
                description: description.to_string(),
            },
        )
        .collect()
}
