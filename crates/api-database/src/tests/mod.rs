mod mutation;
mod query;
mod redis;

use crate::Client;
use anyhow::Result;

async fn create_client(with_ns: Option<&str>) -> Result<Client> {
    dotenvy::dotenv().ok();

    let db_host = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL");
    let db_host = db_host.replace("http://", "");

    let username = std::env::var("TEST_DATABASE_USERNAME").expect("TEST_DATABASE_USERNAME");
    let password = std::env::var("TEST_DATABASE_PASSWORD").expect("TEST_DATABASE_PASSWORD");
    let db_namespace = std::env::var("TEST_DATABASE_NAMESPACE").expect("TEST_DATABASE_NAMESPACE");
    let db_name = std::env::var("TEST_DATABASE_NAME").expect("TEST_DATABASE_NAME");

    let client = Client::try_new(
        &db_host,
        &username,
        &password,
        with_ns.unwrap_or(&db_namespace),
        &db_name,
        None,
        None,
    )
    .await?;

    Ok(client)
}
