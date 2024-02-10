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

#[tokio::test]
async fn gql_search() -> Result<(), Box<dyn std::error::Error>> {
    let schema = super::init_schema().await;

    let res = schema
        .execute(
            r#"
           query {
             search(first: 2, query: "Some Text") {
               edges{
                 cursor
                 node{
                   id,
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

    // search client not configured
    assert!(!res.errors.is_empty());

    let res_name = schema
        .execute(
            r#"
           query {
             searchWithParentName(first: 2, query: "Some Text") {
               edges{
                 cursor
                 node{
                   category {
                       id
                   }
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

    // search client not configured
    assert_eq!(&res.errors[0].message, &res_name.errors[0].message);

    Ok(())
}
