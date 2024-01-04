pub mod env;

use anyhow::{Ok, Result};
use tracing::instrument;

pub struct AppState {
    pub port: u16,
    pub otel_collector_endpoint: String,
}

impl AppState {
    #[instrument(name = "env.cfg")]
    pub fn try_from_env() -> Result<AppState> {
        let port: u16 = env::extract_variable("PORT", "3000").parse()?;
        let otel_collector_endpoint =
            env::extract_variable("OPENTELEMETRY_COLLECTOR_HOST", "http://localhost:4318");

        Ok(AppState {
            port,
            otel_collector_endpoint,
        })
    }
}
