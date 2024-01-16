pub mod env;

use anyhow::{Ok, Result};
use api_interface::DatabaseCredentials;
use metrics_exporter_prometheus::PrometheusHandle;
use tracing::instrument;

use crate::telemetry::metrics::setup_metrics_recorder;

pub struct AppState {
    pub port: u16,
    pub otel_collector_endpoint: String,
    database_dsn: String,
    database_username: String,
    database_password: String,
    database_namespace: String,
    database_name: String,
    pub frontend_url: String,
    pub metrics_handle: PrometheusHandle,
}

impl AppState {
    #[instrument(name = "env.cfg")]
    pub fn try_from_env() -> Result<AppState> {
        let port: u16 = env::extract_variable("PORT", "3000").parse()?;
        let otel_collector_endpoint =
            env::extract_variable("OPENTELEMETRY_COLLECTOR_HOST", "http://localhost:4318");

        let (dsn, db_name, db_user, db_pass, db_ns) = {
            if cfg!(test) {
                (
                    "TEST_DATABASE_URL",
                    "TEST_DATABASE_NAME",
                    "TEST_DATABASE_USERNAME",
                    "TEST_DATABASE_PASSWORD",
                    "TEST_DATABASE_NAMESPACE",
                )
            } else {
                (
                    "DATABASE_DSN",
                    "DATABASE_NAME",
                    "DATABASE_USERNAME",
                    "DATABASE_PASSWORD",
                    "DATABASE_NAMESPACE",
                )
            }
        };

        let database_dsn = env::extract_variable(dsn, "localhost:8000");

        #[cfg(test)]
        let database_dsn = database_dsn.replace("http://", "");

        let database_username = env::extract_variable(db_user, "");
        let database_password = env::extract_variable(db_pass, "");
        let database_namespace = env::extract_variable(db_ns, "");
        let database_name = env::extract_variable(db_name, "");
        let frontend_url = env::extract_variable("FRONTEND_URL", "http://localhost:5173");

        let metrics_handle = setup_metrics_recorder()?;

        Ok(AppState {
            port,
            otel_collector_endpoint,
            database_dsn,
            database_username,
            database_password,
            database_name,
            database_namespace,
            frontend_url,
            metrics_handle,
        })
    }

    pub fn database_credentials(&self) -> DatabaseCredentials {
        DatabaseCredentials {
            db_dsn: &self.database_dsn,
            db_user: &self.database_username,
            db_pass: &self.database_password,
            db_ns: &self.database_namespace,
            db: &self.database_name,
        }
    }
}
