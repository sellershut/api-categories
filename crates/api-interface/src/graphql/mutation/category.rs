use api_core::{api::MutateCategories, Category};
use api_database::Client;
use async_graphql::{Context, Object};
use tracing::error;

#[derive(Default)]
pub struct CategoryMutation;

#[Object]
impl CategoryMutation {
    async fn create_category(
        &self,
        ctx: &Context<'_>,
        input: Category,
    ) -> async_graphql::Result<Category> {
        let database = ctx.data::<Client>()?;

        match database.create_category(&input).await {
            Ok(category) => Ok(category),
            Err(e) => {
                error!("{e}");
                todo!()
            }
        }
    }
}
