pub mod env;

use anyhow::{Ok, Result};
use api_interface::DatabaseCredentials;
use tracing::instrument;

pub struct AppState {
    pub port: u16,
    pub otel_collector_endpoint: String,
    database_dsn: String,
    database_username: String,
    database_password: String,
    database_namespace: String,
    database_name: String,
    pub frontend_url: String,
}

impl AppState {
    #[instrument(name = "env.cfg")]
    pub fn try_from_env() -> Result<AppState> {
        let port: u16 = env::extract_variable("PORT", "3000").parse()?;
        let otel_collector_endpoint =
            env::extract_variable("OPENTELEMETRY_COLLECTOR_HOST", "http://localhost:4318");

        let database_dsn = env::extract_variable("DATABASE_DSN", "localhost:8000");
        let database_username = env::extract_variable("DATABASE_USERNAME", "");
        let database_password = env::extract_variable("DATABASE_PASSWORD", "");
        let database_namespace = env::extract_variable("DATABASE_NAMESPACE", "");
        let database_name = env::extract_variable("DATABASE_NAME", "");
        let frontend_url = env::extract_variable("FRONTEND_URL", "http://localhost:5173");

        Ok(AppState {
            port,
            otel_collector_endpoint,
            database_dsn,
            database_username,
            database_password,
            database_name,
            database_namespace,
            frontend_url,
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
