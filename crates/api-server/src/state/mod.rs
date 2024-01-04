mod env;

use anyhow::{Ok, Result};
use tracing::instrument;

pub struct AppState {
    pub port: u16,
}

impl AppState {
    #[instrument(name = "env.cfg")]
    pub fn try_from_env() -> Result<AppState> {
        let port: u16 = env::extract_variable("PORT", "3000").parse()?;

        Ok(AppState { port })
    }
}
