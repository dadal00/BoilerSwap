use crate::AppError;
use redis::{
    Client,
    aio::{ConnectionManager, ConnectionManagerConfig},
};
use std::env;
use std::time::Duration;
use tracing::warn;

pub async fn init_redis() -> Result<ConnectionManager, AppError> {
    let redis_url = env::var("RUST_REDIS_URL").unwrap_or_else(|_| {
        warn!("Environment variable RUST_REDIS_URL not found, using default");
        "redis://redis:6379".to_string()
    });

    let client = Client::open(redis_url)?;

    let config = ConnectionManagerConfig::new()
        .set_number_of_retries(1)
        .set_connection_timeout(Duration::from_millis(100));

    let connection_manager = client.get_connection_manager_with_config(config).await?;
    Ok(connection_manager)
}
