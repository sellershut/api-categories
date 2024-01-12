use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

use crate::Category;

use super::create_category;

struct Root;

#[Object]
impl Root {
    async fn output(&self) -> Category {
        create_category().unwrap()
    }
}

#[tokio::test]
async fn gql_query() {
    let schema = Schema::new(Root, EmptyMutation, EmptySubscription);

    let res = schema
        .execute(
            r#"
           query {
             output {
               name
             }
           }
           "#,
        )
        .await;

    dbg!(&res);

    assert!(res.errors.is_empty());
}
