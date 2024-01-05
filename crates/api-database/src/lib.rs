use thiserror::Error;

mod mutation;
mod query;

use surrealdb::{
    engine::remote::ws::{Client as SurrealClient, Ws},
    opt::auth::Root,
    Surreal,
};
use tracing::{instrument, trace};

pub struct Client {
    client: Surreal<SurrealClient>,
    cache_ttl: u16,
}

impl Client {
    #[instrument]
    pub async fn try_new(dsn: &str, username: &str, password: &str) -> Result<Self, ClientError> {
        trace!("connecting to database");
        let db = Surreal::new::<Ws>(dsn).await?;

        // Signin as a namespace, database, or root user
        db.signin(Root {
            username,
            password,
        })
        .await?;

        db.use_ns("sellershut").use_db("ads").await?;

        Ok(Client {
            client: db,
            cache_ttl: 5,
        })
    }
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("database engine error")]
    Engine(#[from] surrealdb::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
