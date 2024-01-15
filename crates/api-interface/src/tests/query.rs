#[tokio::test]
async fn gql_query() -> Result<(), Box<dyn std::error::Error>> {
    let schema = super::init_schema().await;

    let res = schema
        .execute(
            r#"
           query {
             categories(first: 2) {
               edges{
                 cursor
                 node{
                   id,
                   name
                 }
               },
               pageInfo {
                 hasNextPage,
                 hasPreviousPage
               }
             }
           }
           "#,
        )
        .await;

    assert!(res.errors.is_empty());

    Ok(())
}

#[tokio::test]
async fn gql_query_sub_categories_ok() {
    let schema = super::init_schema().await;

    let res = schema
        .execute(
            r#"
           query {
             subCategories(first: 2) {
               edges{
                 cursor
                 node{
                   id,
                   name
                 }
               },
               pageInfo {
                 hasNextPage,
                 hasPreviousPage
               }
             }
           }
           "#,
        )
        .await;

    assert!(res.errors.is_empty());
}
