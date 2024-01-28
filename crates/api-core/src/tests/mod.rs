mod async_graphql;
mod db;

use std::str::FromStr;

use crate::{tests::db::SampleDbSend, Category, Id};

use self::db::SampleDb;

fn create_category() -> Category {
    Category {
        #[cfg(feature = "surrealdb")]
        id: Id::Thing(surrealdb::sql::Thing::from(("category", "something"))),
        #[cfg(not(feature = "surrealdb"))]
        id: Id::String("something".to_string()),
        name: String::from("Something"),
        sub_categories: Some(vec![]),
        image_url: None,
        is_root: false,
    }
}

#[test]
fn encode() {
    let category = create_category();

    let json = serde_json::to_string(&category).unwrap();
    dbg!(&json);
    let bytes = bincode::serialize(&category).unwrap();

    let value = serde_json::from_str::<Category>(&json);
    dbg!(&value);

    assert!(value.is_ok());
    let val: Category = bincode::deserialize(&bytes[..]).unwrap();
    assert_eq!(val, category);
}

#[test]
fn deserialise_list() {
    let category = create_category();

    let id = Id::from_str("category:something").expect("created id from str");
    let category_2 = Category {
        id,
        name: "Something".into(),
        sub_categories: None,
        image_url: None,
        is_root: true,
    };

    let categories = vec![category, category_2];

    let str_val = serde_json::to_string(&categories);

    dbg!(&str_val);

    assert!(str_val.is_ok());
}

#[tokio::test]
async fn query_returns() {
    use crate::api::LocalQueryCategories;

    let db = SampleDb.get_categories().await;
    assert!(db.is_ok());

    let mut id = None;
    let db = SampleDb.get_sub_categories(id).await;
    assert!(db.is_ok());

    id = Some("id");
    let db = SampleDb.get_sub_categories(id).await;
    assert!(db.is_ok());

    let db = SampleDb.get_category_by_id("id").await;
    assert!(db.is_ok());

    let db = SampleDb.get_category_by_id("").await;
    assert!(db.is_err());
}

#[tokio::test]
async fn mutation_returns() {
    use crate::api::LocalMutateCategories;

    let category = create_category();

    let db = SampleDb.create_category(&category).await;
    assert!(db.is_ok());

    let db = SampleDb.update_category("some:id", &category).await;
    assert!(db.is_ok());

    let db = SampleDb.delete_category("something:else").await;
    assert!(db.is_ok());
}

#[tokio::test]
async fn mutation_returns_send() {
    use crate::api::MutateCategories;

    let category = create_category();

    let db = SampleDbSend.create_category(&category).await;
    assert!(db.is_ok());

    let db = SampleDbSend.update_category("some:id", &category).await;
    assert!(db.is_ok());

    let db = SampleDbSend.delete_category("something:else").await;
    assert!(db.is_ok());
}

#[tokio::test]
async fn query_returns_send() {
    use crate::api::QueryCategories;

    let db = SampleDbSend.get_categories().await;
    assert!(db.is_ok());

    let mut id = None;
    let db = SampleDbSend.get_sub_categories(id).await;
    assert!(db.is_ok());

    id = Some("id");
    let db = SampleDbSend.get_sub_categories(id).await;
    assert!(db.is_ok());

    let db = SampleDbSend.get_category_by_id("id").await;
    assert!(db.is_ok());

    let db = SampleDbSend.get_category_by_id("").await;
    assert!(db.is_err());
}
