use crate::config::Config;
use crate::metrics::Metrics;

pub struct AppState {
    pub config: Config,
    pub metrics: Metrics,
}
