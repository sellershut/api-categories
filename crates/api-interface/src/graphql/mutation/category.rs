use api_core::{api::MutateCategories, Category};
use api_database::Client;
use async_graphql::{Context, Object};

use crate::graphql::subscription::{broker::SimpleBroker, CategoryChanged};

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
            Ok(category) => {
                SimpleBroker::publish(CategoryChanged {
                    mutation_type: super::MutationType::Created,
                    id: category.id.clone().into(),
                });

                Ok(category)
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn update_category(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: Category,
    ) -> async_graphql::Result<Option<Category>> {
        let database = ctx.data::<Client>()?;

        match database.update_category(&id, &input).await {
            Ok(category) => {
                SimpleBroker::publish(CategoryChanged {
                    mutation_type: super::MutationType::Updated,
                    id: id.into(),
                });
                Ok(category)
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn delete_category(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<Option<Category>> {
        let database = ctx.data::<Client>()?;

        match database.delete_category(&id).await {
            Ok(category) => {
                SimpleBroker::publish(CategoryChanged {
                    mutation_type: super::MutationType::Deleted,
                    id: id.into(),
                });
                Ok(category)
            }
            Err(e) => Err(e.into()),
        }
    }
}
