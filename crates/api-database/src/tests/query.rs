use crate::tests::create_client;
use anyhow::Result;
use api_core::{api::QueryCategories, Category};

async fn check_categories_by_id(id: &str, expected_result: bool) -> Result<()> {
    let client = create_client(None).await?;

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
    let client = create_client(None).await?;

    let res = client.get_categories().await;

    assert_eq!(res.is_ok(), expected_result);

    Ok(())
}

async fn check_sub_categories(id: Option<&str>, expected_result: bool) -> Result<()> {
    let client = create_client(None).await?;

    match client.get_sub_categories(id).await {
        Ok(categories) => {
            let categories: Vec<_> = categories.collect();
            if !categories.is_empty() {
                for category in categories {
                    assert_eq!(expected_result, category.parent_id.is_none());
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

#[tokio::test]
async fn query_by_unavailable_id() -> Result<()> {
    check_categories_by_id("justanid", false).await?;
    check_categories_by_id("", false).await?;
    check_categories_by_id("  ", false).await?;

    Ok(())
}

#[tokio::test]
async fn query_by_available_id() -> Result<()> {
    let client = create_client(None).await?;

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
    check_sub_categories(Some("justanid"), true).await?;

    Ok(())
}
