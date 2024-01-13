use super::create_client;
use anyhow::Result;
use api_core::{
    api::{MutateCategories, QueryCategories},
    Category, Id,
};

fn create_category_item() -> Category {
    Category {
        id: Id::default(),
        name: "TestCategoryInput".into(),
        sub_categories: None,
        image_url: None,
        is_root: true,
    }
}

fn check_similarities(source: &Category, dest: &Category) {
    assert_eq!(source.name, dest.name);
    assert_eq!(source.sub_categories, dest.sub_categories);
    assert_eq!(source.is_root, dest.is_root);
}

#[tokio::test]
async fn create_category() -> Result<()> {
    let client = create_client(Some("test-mutation-create")).await?;

    let all_categories = client.get_categories().await?;

    let base_count = all_categories.count();
    let category = create_category_item();

    let input = client.create_category(&category).await?;

    let updated_categories = client.get_categories().await?;

    assert_eq!(base_count + 1, updated_categories.count());
    check_similarities(&input, &category);

    Ok(())
}

#[tokio::test]
async fn create_get_by_id() -> Result<()> {
    let category = create_category_item();

    let client = create_client(Some("test-mutation-update")).await?;

    let input = client.create_category(&category).await?;

    let get_by_id = client.get_category_by_id(&input.id.to_string()).await?;
    assert_eq!(get_by_id, Some(input));

    Ok(())
}

#[tokio::test]
async fn update_no_id() -> Result<()> {
    let mut update = create_category_item();

    let client = create_client(Some("test-mutation-bad-id")).await?;

    update.name = "FooBar".to_string();
    update.id = Id::default();
    update.is_root = false;

    // Empty IDs return errors
    let update_res = client
        .update_category(&update.id.to_string(), &update)
        .await;

    assert!(update_res.is_err());

    Ok(())
}

#[tokio::test]
async fn update_category() -> Result<()> {
    let category = create_category_item();

    let client = create_client(Some("test-mutation-update")).await?;

    let input = client.create_category(&category).await?;

    let mut update = input.clone();
    update.name = "FooBar".to_string();
    update.is_root = false;

    // This ID does exist
    let update_res = client
        .update_category(&input.id.to_string(), &update)
        .await?
        .expect("category to exist in db");

    assert_eq!(update_res.id, input.id);
    check_similarities(&update, &update_res);

    Ok(())
}

#[tokio::test]
async fn delete_category() -> Result<()> {
    let category = create_category_item();
    let client = create_client(Some("test-mutation-update")).await?;

    let all_categories = client.get_categories().await?;

    let base_count = all_categories.count();

    let input = client.create_category(&category).await?;
    // delete and check count
    let deleted_category = client
        .delete_category(&input.id.to_string())
        .await?
        .expect("category to be deleted");

    assert_eq!(input, deleted_category);

    let final_count = client.get_categories().await?.count();
    assert_eq!(base_count, final_count);

    Ok(())
}