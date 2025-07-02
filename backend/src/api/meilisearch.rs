use super::{
    database::{DatabaseQueries, convert_db_items},
    models::ItemRow,
    schema::{columns::items, tables},
};
use crate::{AppError, config::read_secret};
use meilisearch_sdk::client::*;
use scylla::{client::session::Session, response::PagingState};
use serde::Serialize;
use std::{
    env,
    marker::{Send, Sync},
    ops::ControlFlow,
    sync::Arc,
};
use tokio::task::JoinHandle;
use tracing::warn;
use uuid::Uuid;

pub async fn init_meilisearch(
    database_session: Arc<Session>,
    database_queries: &DatabaseQueries,
) -> Result<(Arc<Client>, JoinHandle<Result<(), AppError>>), AppError> {
    let meili_url = env::var("MEILI_URL").unwrap_or_else(|_| {
        warn!("Environment variable MEILI_URL not found, using default");
        "http://meilisearch:7700".to_string()
    });
    let meili_client =
        Arc::new(Client::new(meili_url, Some(read_secret("MEILI_ADMIN_KEY")?)).unwrap());

    let client_clone = meili_client.clone();
    let session_clone = database_session.clone();
    let queries_clone = database_queries.clone();

    let reindex_future = tokio::spawn(async move {
        reindex(
            session_clone,
            queries_clone,
            client_clone,
            tables::ITEMS,
            items::ITEM_ID,
        )
        .await
    });

    Ok((meili_client, reindex_future))
}

pub async fn reindex(
    database_session: Arc<Session>,
    database_queries: DatabaseQueries,
    meili_client: Arc<Client>,
    index_name: &str,
    item_id_name: &str,
) -> Result<(), AppError> {
    let mut paging_state = PagingState::start();

    loop {
        let (query_result, paging_state_response) = database_session
            .execute_single_page(&database_queries.get_items, &[], paging_state)
            .await?;

        let row_result = query_result.into_rows_result()?;

        let row_vec: Vec<ItemRow> = row_result
            .rows::<ItemRow>()?
            .collect::<Result<Vec<_>, _>>()?;

        add_items(
            meili_client.clone(),
            index_name,
            &convert_db_items(&row_vec),
            item_id_name,
        )
        .await?;

        match paging_state_response.into_paging_control_flow() {
            ControlFlow::Break(()) => {
                break Ok(());
            }
            ControlFlow::Continue(new_paging_state) => paging_state = new_paging_state,
        }
    }
}

pub async fn add_items<T>(
    meili_client: Arc<Client>,
    index_name: &str,
    items: &[T],
    id_name: &str,
) -> anyhow::Result<()>
where
    T: Serialize + Send + Sync,
{
    meili_client
        .index(index_name)
        .add_documents(items, Some(id_name))
        .await?
        .wait_for_completion(&meili_client, None, None)
        .await?;
    Ok(())
}

pub async fn delete_item(
    meili_client: Arc<Client>,
    index_name: &str,
    key: Uuid,
) -> anyhow::Result<()> {
    meili_client
        .index(index_name)
        .delete_document(key)
        .await?
        .wait_for_completion(&meili_client, None, None)
        .await?;
    Ok(())
}
