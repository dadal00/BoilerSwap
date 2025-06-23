use crate::{
    api::{
        database::{DatabaseQueries, init_database},
        redis::init_redis,
    },
    config::Config,
    error::AppError,
    metrics::Metrics,
};
use redis::aio::ConnectionManager;
use scylla::client::session::Session;
use std::sync::Arc;

pub struct AppState {
    pub config: Config,
    pub metrics: Metrics,
    pub database_session: Arc<Session>,
    pub database_queries: DatabaseQueries,
    pub redis_connection_manager: ConnectionManager,
}

impl AppState {
    pub async fn new() -> Result<Arc<Self>, AppError> {
        let database_future = init_database();
        let redis_future = init_redis();

        let config = Config::load()?;
        let metrics = Metrics::default();

        let redis_connection_manager = redis_future.await?;
        let (database_session, database_queries) = database_future.await?;

        Ok(Arc::new(Self {
            config,
            metrics,
            database_session,
            database_queries,
            redis_connection_manager,
        }))
    }
}
