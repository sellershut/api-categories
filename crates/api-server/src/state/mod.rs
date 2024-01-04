mod env;

use anyhow::{Ok, Result};
use tracing::{instrument, warn};

pub struct AppState {
    pub port: u16,
}

impl AppState {
    #[instrument]
    pub fn try_from_env() -> Result<AppState> {
        dotenvy::dotenv().ok();

        let port: u16 = env::extract_variable("PORT", "3000").parse()?;

        Ok(AppState { port })
    }
}
