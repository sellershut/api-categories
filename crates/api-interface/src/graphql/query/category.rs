use api_core::{api::QueryCategories, Category};
use api_database::Client;
use async_graphql::{Context, Object};

#[derive(Default, Debug)]
pub struct CategoryQuery;

#[Object]
impl CategoryQuery {
    async fn categories(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Vec<Category> {
        let database = ctx.data::<Client>().unwrap();

        let categories = database.get_categories().await.unwrap();

        categories.collect()
    }

    async fn category_by_id(&self, ctx: &Context<'_>, id: String) -> Option<Category> {
        let database = ctx.data::<Client>().unwrap();

        database.get_category_by_id(&id).await.unwrap()
    }
}
