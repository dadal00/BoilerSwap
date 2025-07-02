use super::{
    database::{DatabaseQueries, convert_db_items},
    models::ItemRow,
};
use crate::{AppError, config::read_secret};
use meilisearch_sdk::client::*;
use scylla::{client::session::Session, response::PagingState};
use std::{env, ops::ControlFlow, sync::Arc};
use tokio::task::JoinHandle;
use tracing::warn;

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

    let reindex_future =
        tokio::spawn(async move { reindex(session_clone, queries_clone, client_clone).await });

    Ok((meili_client, reindex_future))
}

pub async fn reindex(
    database_session: Arc<Session>,
    database_queries: DatabaseQueries,
    meili_client: Arc<Client>,
) -> Result<(), AppError> {
    let items_index = meili_client.index("items");

    let mut paging_state = PagingState::start();

    loop {
        let (query_result, paging_state_response) = database_session
            .execute_single_page(&database_queries.get_items, &[], paging_state)
            .await?;

        let row_result = query_result.into_rows_result()?;

        let row_vec: Vec<ItemRow> = row_result
            .rows::<ItemRow>()?
            .collect::<Result<Vec<_>, _>>()?;

        items_index
            .add_documents(&convert_db_items(&row_vec), Some("item_id"))
            .await?
            .wait_for_completion(&meili_client, None, None)
            .await?;

        match paging_state_response.into_paging_control_flow() {
            ControlFlow::Break(()) => {
                break Ok(());
            }
            ControlFlow::Continue(new_paging_state) => paging_state = new_paging_state,
        }
    }
}
