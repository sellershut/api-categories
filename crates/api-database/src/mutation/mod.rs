use std::fmt::Debug;

use api_core::{
    api::{CoreError, MutateCategories},
    Category, Id,
};
use surrealdb::sql::Thing;
use tracing::{debug, instrument};

use crate::{collections::Collections, Client};

impl MutateCategories for Client {
    #[instrument(skip(self))]
    async fn create_category(&self, category: &Category) -> Result<Category, CoreError> {
        debug!("creating category");

        let input_category = InputCategory::from(category);

        let item: Vec<Category> = self
            .client
            .create("category")
            .content(input_category)
            .await?;

        match item.into_iter().nth(0) {
            Some(category) => Ok(category),
            None => unreachable!("create returned no elements"),
        }
    }

    #[instrument(skip(self, id))]
    async fn update_category(
        &self,
        id: impl AsRef<str> + Send + Debug,
        data: &Category,
    ) -> Result<Option<Category>, CoreError> {
        debug!("updating category");
        let id = Thing::from((Collections::Category.to_string().as_str(), id.as_ref()));

        let item: Option<Category> = self.client.update(id).content(data).await?;

        Ok(item)
    }

    async fn delete_category(
        &self,
        id: impl AsRef<str> + Send + Debug,
    ) -> Result<Option<Category>, CoreError> {
        debug!("deleting category");

        let id = Thing::from((Collections::Category.to_string().as_str(), id.as_ref()));

        let res = self.client.delete(id).await?;

        Ok(res)
    }
}

#[derive(serde::Serialize)]
struct InputCategory<'a> {
    name: &'a str,
    sub_categories: Option<&'a [Id]>,
    image_url: Option<&'a str>,
    is_root: bool,
}

impl<'a> From<&'a Category> for InputCategory<'a> {
    fn from(value: &'a Category) -> Self {
        Self {
            name: &value.name,
            sub_categories: value.sub_categories.as_deref(),
            image_url: value.image_url.as_deref(),
            is_root: value.is_root,
        }
    }
}
