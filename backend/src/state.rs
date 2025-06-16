use crate::{
    config::Config, database::init_database, error::AppError, metrics::Metrics, redis::init_redis,
};
use redis::aio::ConnectionManager;
use scylla::{client::session::Session, statement::prepared::PreparedStatement};
use std::sync::Arc;

pub struct AppState {
    pub config: Config,
    pub metrics: Metrics,
    pub database_session: Arc<Session>,
    pub database_queries: DatabaseQueries,
    pub redis_connection_manager: ConnectionManager,
}

pub struct DatabaseQueries {
    pub get_user_info: PreparedStatement,
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
