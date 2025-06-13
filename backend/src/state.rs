use crate::{config::Config, db::init_db, error::AppError, metrics::Metrics};
use scylla::client::session::Session;
use std::sync::Arc;

pub struct AppState {
    pub config: Config,
    pub metrics: Metrics,
    pub db_session: Arc<Session>,
}

impl AppState {
    pub async fn new() -> Result<Arc<Self>, AppError> {
        let config = Config::load()?;
        let metrics = Metrics::default();
        let db_session = init_db().await?;

        Ok(Arc::new(Self {
            config,
            metrics,
            db_session,
        }))
    }
}
