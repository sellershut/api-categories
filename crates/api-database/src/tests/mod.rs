use crate::Client;
use anyhow::Result;
use api_core::{api::QueryCategories, Category};

async fn check_categories_by_id(id: &str, expected_result: bool) -> Result<()> {
    let client = create_client().await?;

    match client.get_category_by_id(id).await {
        Ok(res) => {
            assert_eq!(res.is_some(), expected_result);
        }
        Err(_) => {
            assert!(!expected_result);
        }
    }

    Ok(())
}

async fn check_all(expected_result: bool) -> Result<()> {
    let client = create_client().await?;

    let res = client.get_categories().await;

    assert_eq!(res.is_ok(), expected_result);

    Ok(())
}

async fn check_sub_categories(id: Option<&str>, expected_result: bool) -> Result<()> {
    let client = create_client().await?;

    match client.get_sub_categories(id).await {
        Ok(categories) => {
            let categories: Vec<_> = categories.collect();
            if !categories.is_empty() {
                for category in categories {
                    assert_eq!(expected_result, category.is_root);
                }
            } else {
                assert!(expected_result);
            }
        }
        Err(_) => {
            assert!(!expected_result);
        }
    }

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

#[tokio::test]
async fn query_by_unavailable_id() -> Result<()> {
    check_categories_by_id("justanid", false).await?;
    check_categories_by_id("", false).await?;
    check_categories_by_id("  ", false).await?;

    Ok(())
}

#[tokio::test]
async fn query_by_available_id() -> Result<()> {
    let client = create_client().await?;

    let mut res = client
        .client
        .query("SELECT * FROM category LIMIT 5;")
        .await?;

    let resp: Vec<Category> = res.take(0)?;

    if let Some(item) = resp.first() {
        check_categories_by_id(&item.id.to_string(), true).await?;
    }

    Ok(())
}

#[tokio::test]
async fn query_all() -> Result<()> {
    check_all(true).await?;

    Ok(())
}

#[tokio::test]
async fn query_sub_categories() -> Result<()> {
    check_sub_categories(None, true).await?;

    Ok(())
}
