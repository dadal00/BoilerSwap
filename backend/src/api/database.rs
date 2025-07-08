use super::{
    consumer::MeiliConsumerFactory,
    models::{Condition, Emoji, Item, ItemPayload, ItemRow, ItemType, Location, RedisAccount},
    schema::{
        KEYSPACE,
        columns::{items, users},
        tables,
    },
    utilities::convert_i8_to_u8,
};
use crate::{error::AppError, state::AppState};
use anyhow::Error as anyhowError;
use chrono::{Duration as chronoDuration, NaiveDate, Utc};
use futures_util::future::RemoteHandle;
use once_cell::sync::Lazy;
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

static BASE_DATE: Lazy<NaiveDate> = Lazy::new(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

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
            format!("CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{'class': 'SimpleStrategy', 'replication_factor': 1}}", KEYSPACE),
            &[],
        )
        .await?;

    database_session
        .query_unpaged(
            format!(
                "CREATE TABLE IF NOT EXISTS {}.{} (
                {} {},
                {} {},
                {} {},
                PRIMARY KEY({})
            )",
                KEYSPACE,
                tables::USERS,
                users::EMAIL,
                users::EMAIL_TYPE,
                users::PASSWORD_HASH,
                users::PASSWORD_HASH_TYPE,
                users::LOCKED,
                users::LOCKED_TYPE,
                users::PRIMARY_KEY,
            ),
            &[],
        )
        .await?;

    database_session
        .query_unpaged(
            format!(
                "CREATE TABLE IF NOT EXISTS {}.{} (
                {} {},
                {} {},
                {} {},
                {} {},
                {} {},
                {} {},
                {} {},
                {} {},
                PRIMARY KEY({})
            ) WITH cdc = {{'enabled': true}}",
                KEYSPACE,
                tables::ITEMS,
                items::ITEM_ID,
                items::ITEM_ID_TYPE,
                items::ITEM_TYPE,
                items::ITEM_TYPE_TYPE,
                items::TITLE,
                items::TITLE_TYPE,
                items::CONDITION,
                items::CONDITION_TYPE,
                items::LOCATION,
                items::LOCATION_TYPE,
                items::DESCRIPTION,
                items::DESCRIPTION_TYPE,
                items::EMOJI,
                items::EMOJI_TYPE,
                items::EXPIRATION_DATE,
                items::EXPIRATION_DATE_TYPE,
                items::PRIMARY_KEY,
            ),
            &[],
        )
        .await?;

    let database_queries = DatabaseQueries {
        get_user: database_session
            .prepare(format!(
                "SELECT {}, {} FROM {}.{} WHERE {} = ?",
                users::PASSWORD_HASH,
                users::LOCKED,
                KEYSPACE,
                tables::USERS,
                users::PRIMARY_KEY
            ))
            .await?,
        insert_user: database_session
            .prepare(format!(
                "INSERT INTO {}.{} ({}, {}, {}) VALUES (?, ?, ?) USING TTL {}",
                KEYSPACE,
                tables::USERS,
                users::EMAIL,
                users::PASSWORD_HASH,
                users::LOCKED,
                users::TTL
            ))
            .await?,
        check_lock: database_session
            .prepare(format!(
                "SELECT {} FROM {}.{} WHERE {} = ?",
                users::LOCKED,
                KEYSPACE,
                tables::USERS,
                users::PRIMARY_KEY
            ))
            .await?,
        update_lock: database_session
            .prepare(format!(
                "UPDATE {}.{} SET {} = ? WHERE {} = ?",
                KEYSPACE,
                tables::USERS,
                users::LOCKED,
                users::PRIMARY_KEY
            ))
            .await?,
        unlock_account: database_session
            .prepare(format!(
                "UPDATE {}.{} SET {} = false, {} = ? WHERE {} = ?",
                KEYSPACE,
                tables::USERS,
                users::LOCKED,
                users::PASSWORD_HASH,
                users::PRIMARY_KEY
            ))
            .await?,
        insert_item: database_session
            .prepare(format!(
                "INSERT INTO {}.{} ({}, {}, {}, {}, {}, {}, {}, {}) VALUES (?, ?, ?, ?, ?, ?, ?, ?) USING TTL ?",
                KEYSPACE,
                tables::ITEMS,
                items::ITEM_ID,
                items::ITEM_TYPE,
                items::TITLE,
                items::CONDITION,
                items::LOCATION,
                items::DESCRIPTION,
                items::EMOJI,
                items::EXPIRATION_DATE,
            ))
            .await?,
        get_items: database_session
            .prepare(
                Statement::new(format!(
                    "SELECT {}, {}, {}, {}, {}, {}, {}, {} FROM {}.{}", 
                    items::ITEM_ID,
                    items::ITEM_TYPE,
                    items::TITLE,
                    items::CONDITION,
                    items::LOCATION,
                    items::DESCRIPTION,
                    items::EMOJI,
                    items::EXPIRATION_DATE,
                    KEYSPACE,
                    tables::ITEMS
                )).with_page_size(100),
            )
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
                item.emoji as i8,
                Utc::now().date_naive() + chronoDuration::days(7),
                604800,
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
            |(
                id,
                item_type_i8,
                title,
                condition_i8,
                location_i8,
                description,
                emoji_i8,
                expiration_date,
            )| Item {
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
                emoji: Emoji::try_from(convert_i8_to_u8(emoji_i8))
                    .unwrap_or(Emoji::Books)
                    .as_ref()
                    .to_string(),
                expiration_date: expiration_date.format("%Y-%m-%d").to_string(),
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
            tables::CDC,
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

pub fn get_cdc_id(data: &CDCRow<'_>, id_name: &str) -> Uuid {
    data.get_value(id_name)
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

pub fn get_cdc_date(data: &CDCRow<'_>, column: &str) -> String {
    let days = data
        .get_value(column)
        .as_ref()
        .and_then(|v| v.as_cql_date())
        .expect("Missing date attribute")
        .0 as i64;

    BASE_DATE
        .checked_add_signed(chronoDuration::days(days - 2_147_483_648))
        .map(|d| d.format("%Y-%m-%d").to_string())
        .expect("Missing the date attribute!")
}

pub fn convert_cdc_item(data: CDCRow<'_>) -> Item {
    Item {
        item_id: get_cdc_id(&data, items::ITEM_ID),
        item_type: ItemType::try_from(get_cdc_u8(&data, items::ITEM_TYPE))
            .unwrap_or(ItemType::Other)
            .as_ref()
            .to_string(),
        title: get_cdc_text(&data, items::TITLE),
        condition: Condition::try_from(get_cdc_u8(&data, items::CONDITION))
            .unwrap_or(Condition::Fair)
            .as_ref()
            .to_string(),
        location: Location::try_from(get_cdc_u8(&data, items::LOCATION))
            .unwrap_or(Location::CaryQuadEast)
            .as_ref()
            .to_string(),
        description: get_cdc_text(&data, items::DESCRIPTION),
        emoji: Emoji::try_from(get_cdc_u8(&data, items::EMOJI))
            .unwrap_or(Emoji::Books)
            .as_ref()
            .to_string(),
        expiration_date: get_cdc_date(&data, items::EXPIRATION_DATE),
    }
}
