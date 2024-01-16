use api_core::{api::QueryCategories, Category};
use async_graphql::{Context, Object};
use tracing::instrument;

use crate::graphql::{extract_db, query::Params};

use super::{pagination::paginate, ConnectionResult};

#[derive(Default, Debug)]
pub struct CategoryQuery;

#[Object]
impl CategoryQuery {
    #[instrument(skip(ctx), err(Debug))]
    async fn categories(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1, max_length = 100))] after: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> ConnectionResult<Category> {
        let p = Params::new(after, before, first, last)?;

        let database = extract_db(ctx)?;

        let categories = database.get_categories().await?;

        paginate(categories, p, 100).await
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn sub_categories(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1, max_length = 100))] parent_id: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] after: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> ConnectionResult<Category> {
        let p = Params::new(after, before, first, last)?;

        let database = extract_db(ctx)?;

        let categories = database
            .get_sub_categories(parent_id.as_deref())
            .await
            .unwrap();

        paginate(categories, p, 100).await
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn category_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1, max_length = 100))] id: String,
    ) -> async_graphql::Result<Option<Category>> {
        let database = extract_db(ctx)?;

        database.get_category_by_id(&id).await.map_err(|e| e.into())
    }
}
