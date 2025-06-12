use crate::{AppState, error::AppError};
use axum::extract::State;
use prometheus::{Encoder, IntCounter, Registry, TextEncoder, register_int_counter};
use std::sync::Arc;
use tracing::debug;

#[derive(Debug)]
pub struct Metrics {
    pub dummy: IntCounter,
    registry: Registry,
}

impl Default for Metrics {
    fn default() -> Self {
        let registry = Registry::new();

        let dummy =
            register_int_counter!("dummy", "Dummy metric").expect("Can't create dummy metric");

        registry.register(Box::new(dummy.clone())).unwrap();

        Metrics { dummy, registry }
    }
}

impl Metrics {
    pub fn gather(&self) -> Result<String, AppError> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = vec![];
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

pub async fn metrics_handler(State(state): State<Arc<AppState>>) -> Result<String, AppError> {
    debug!("Metrics being scrapped");
    state.metrics.dummy.inc();
    state.metrics.gather()
}
