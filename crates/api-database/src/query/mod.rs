use api_core::{
    api::{CoreError, QueryCategories},
    Category,
};
use surrealdb::sql::Thing;

use crate::{collections::Collections, Client};

impl QueryCategories for Client {
    async fn get_categories(&self) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        let categories: Vec<Category> = self.client.select(Collections::Category).await?;
        Ok(categories.into_iter())
    }

    async fn get_sub_categories(
        &self,
        id: Option<impl AsRef<str> + Send>,
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

    async fn get_category_by_id(
        &self,
        id: impl AsRef<str> + Send,
    ) -> Result<Option<Category>, CoreError> {
        let id = Thing::from((Collections::Category.to_string().as_str(), id.as_ref()));

        let category = self.client.select(id).await?;

        Ok(category)
    }
}
