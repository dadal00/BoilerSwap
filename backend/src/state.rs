use crate::{config::Config, database::init_database, error::AppError, metrics::Metrics, api::VerificationInfo};
use scylla::{client::session::Session, statement::prepared::PreparedStatement};
use std::{time::Duration,sync::Arc};
use moka::future::Cache;
use uuid::Uuid;

pub struct AppState {
    pub config: Config,
    pub metrics: Metrics,
    pub database_session: Arc<Session>,
    pub database_queries: DatabaseQueries,
    pub verification_map: Arc<Cache<Uuid, VerificationInfo>>,
}

pub struct DatabaseQueries {
    pub get_user_id: PreparedStatement,
}

impl AppState {
    pub async fn new() -> Result<Arc<Self>, AppError> {
        let config = Config::load()?;
        let metrics = Metrics::default();
        let (database_session, database_queries) = init_database().await?;
        let verification_map = Arc::new(
            Cache::builder()
                .time_to_live(Duration::from_secs(300))
                .build()
        );

        Ok(Arc::new(Self {
            config,
            metrics,
            database_session,
            database_queries,
            verification_map,
        }))
    }
}
