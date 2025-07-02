use super::{
    consumer::MeiliConsumerFactory,
    models::{Condition, Item, ItemPayload, ItemRow, ItemType, Location, RedisAccount},
    utilities::{convert_i8_to_u8, get_seconds_until},
};
use crate::{error::AppError, state::AppState};
use anyhow::Error as anyhowError;
use chrono::Weekday;
use futures_util::future::RemoteHandle;
use scylla::{
    client::{session::Session, session_builder::SessionBuilder},
    response::{PagingState, query_result::FirstRowError::RowsEmpty},
    statement::{prepared::PreparedStatement, unprepared::Statement},
};
use scylla_cdc::{
    checkpoints::TableBackedCheckpointSaver,
    consumer::CDCRow,
    log_reader::{CDCLogReader, CDCLogReaderBuilder},
};
use std::{env, sync::Arc, time::Duration};
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
                item_type: ItemType::try_from(convert_i8_to_u8(item_type_i8))
                    .unwrap_or(ItemType::Other)
                    .as_ref()
                    .to_string(),
                title: title.to_string(),
                condition: Condition::try_from(convert_i8_to_u8(condition_i8))
                    .unwrap_or(Condition::Fair)
                    .as_ref()
                    .to_string(),
                location: Location::try_from(convert_i8_to_u8(location_i8))
                    .unwrap_or(Location::CaryQuadEast)
                    .as_ref()
                    .to_string(),
                description: description.to_string(),
            },
        )
        .collect()
}

pub async fn start_cdc(
    state: Arc<AppState>,
    scylla_keyspace: &str,
    scylla_table: &str,
    scylla_id_name: &str,
) -> Result<(CDCLogReader, RemoteHandle<Result<(), anyhowError>>), AppError> {
    let items_checkpoint_saver = Arc::new(
        TableBackedCheckpointSaver::new_with_default_ttl(
            state.database_session.clone(),
            scylla_keyspace,
            scylla_table,
        )
        .await
        .unwrap(),
    );

    let (cdc_reader, cdc_future) = CDCLogReaderBuilder::new()
        .session(state.database_session.clone())
        .keyspace(scylla_keyspace)
        .table_name(scylla_table)
        .should_save_progress(true)
        .should_load_progress(true)
        .window_size(Duration::from_secs(60))
        .safety_interval(Duration::from_secs(30))
        .sleep_interval(Duration::from_secs(10))
        .pause_between_saves(Duration::from_secs(10))
        .consumer_factory(Arc::new(MeiliConsumerFactory {
            meili_client: state.meili_client.clone(),
            meili_index: scylla_table.to_string(),
            scylla_id_name: scylla_id_name.to_string(),
        }))
        .checkpoint_saver(items_checkpoint_saver)
        .build()
        .await?;

    Ok((cdc_reader, cdc_future))
}

pub fn get_cdc_id(data: &CDCRow<'_>) -> Uuid {
    data.get_value("item_id")
        .as_ref()
        .and_then(|v| v.as_uuid())
        .expect("Missing item id")
}

pub fn get_cdc_tinyint(data: &CDCRow<'_>, column: &str) -> i8 {
    data.get_value(column)
        .as_ref()
        .and_then(|v| v.as_tinyint())
        .expect("Missing tinyint attribute")
}

pub fn get_cdc_u8(data: &CDCRow<'_>, column: &str) -> u8 {
    convert_i8_to_u8(&get_cdc_tinyint(data, column))
}

pub fn get_cdc_text(data: &CDCRow<'_>, column: &str) -> String {
    data.get_value(column)
        .as_ref()
        .and_then(|v| v.as_text())
        .expect("Missing text attribute")
        .to_string()
}

pub fn convert_cdc_item(data: CDCRow<'_>) -> Item {
    Item {
        item_id: get_cdc_id(&data),
        item_type: ItemType::try_from(get_cdc_u8(&data, "item_type"))
            .unwrap_or(ItemType::Other)
            .as_ref()
            .to_string(),
        title: get_cdc_text(&data, "title"),
        condition: Condition::try_from(get_cdc_u8(&data, "condition"))
            .unwrap_or(Condition::Fair)
            .as_ref()
            .to_string(),
        location: Location::try_from(get_cdc_u8(&data, "location"))
            .unwrap_or(Location::CaryQuadEast)
            .as_ref()
            .to_string(),
        description: get_cdc_text(&data, "description"),
    }
}
