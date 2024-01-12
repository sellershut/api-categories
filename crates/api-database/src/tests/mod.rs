use crate::Client;
use anyhow::Result;
use api_core::api::QueryCategories;

async fn check_categories_by_id(id: &str, expected_result: bool) -> Result<()> {
    let client = create_client().await?;

    let categories = client.get_category_by_id(id).await;

    dbg!(&categories);

    assert_eq!(categories.is_ok(), expected_result);

    Ok(())
}

async fn create_client() -> Result<Client> {
    dotenvy::dotenv().ok();

    let db_host = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL");
    let username = std::env::var("TEST_DATABASE_USERNAME").expect("TEST_DATABASE_USERNAME");
    let password = std::env::var("TEST_DATABASE_PASSWORD").expect("TEST_DATABASE_PASSWORD");
    let db_namespace = std::env::var("TEST_DATABASE_NAMESPACE").expect("TEST_DATABASE_NAMESPACE");
    let db_name = std::env::var("TEST_DATABASE_NAME").expect("TEST_DATABASE_NAME");

    let client = Client::try_new(&db_host, &username, &password, &db_namespace, &db_name).await?;

    Ok(client)
}

async fn query_categories_by_id() -> Result<()> {
    check_categories_by_id("justanid", false).await?;

    Ok(())
}
