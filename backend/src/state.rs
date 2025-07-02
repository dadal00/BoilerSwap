use crate::{
    api::{
        database::{DatabaseQueries, init_database},
        meilisearch::init_meilisearch,
        redis::init_redis,
    },
    config::Config,
    error::AppError,
    metrics::Metrics,
};
use meilisearch_sdk::client::Client;
use redis::aio::ConnectionManager;
use scylla::client::session::Session;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub struct AppState {
    pub config: Config,
    pub metrics: Metrics,
    pub database_session: Arc<Session>,
    pub database_queries: DatabaseQueries,
    pub redis_connection_manager: ConnectionManager,
    pub meili_client: Arc<Client>,
}

impl AppState {
    pub async fn new() -> Result<(Arc<Self>, JoinHandle<Result<(), AppError>>), AppError> {
        let (database_session, database_queries) = init_database().await?;
        let meili_future = init_meilisearch(database_session.clone(), &database_queries);
        let redis_future = init_redis();

        let config = Config::load()?;
        let metrics = Metrics::default();

        let redis_connection_manager = redis_future.await?;
        let (meili_client, meili_reindex_future) = meili_future.await?;

        Ok((
            Arc::new(Self {
                config,
                metrics,
                database_session,
                database_queries,
                redis_connection_manager,
                meili_client,
            }),
            meili_reindex_future,
        ))
    }
}
