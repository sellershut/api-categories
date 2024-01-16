use std::fmt::Debug;

use api_core::{
    api::{CoreError, QueryCategories},
    Category,
};
use surrealdb::sql::Thing;
use tracing::instrument;

use crate::{collections::Collections, Client};

impl QueryCategories for Client {
    #[instrument(skip(self), err(Debug))]
    async fn get_categories(&self) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        let categories: Vec<Category> = self.client.select(Collections::Category).await?;
        Ok(categories.into_iter())
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_sub_categories(
        &self,
        id: Option<impl AsRef<str> + Send + Debug>,
    ) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        let mut resp = self
            .client
            .query(if let Some(parent) = id {
                format!("SELECT sub_categories.*.* FROM {};", parent.as_ref())
            } else {
                format!("SELECT * FROM {} WHERE is_root=true", Collections::Category)
            })
            .await?;

        let categories: Vec<Category> = resp.take(0)?;

        Ok(categories.into_iter())
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_category_by_id(
        &self,
        id: impl AsRef<str> + Send + Debug,
    ) -> Result<Option<Category>, CoreError> {
        let id = id.as_ref();
        if id.is_empty() {
            return Err(CoreError::Other("Id cannot be empty".into()));
        }
        let id = Thing::from((Collections::Category.to_string().as_str(), id));

        let category = self.client.select(id).await?;

        Ok(category)
    }
}
