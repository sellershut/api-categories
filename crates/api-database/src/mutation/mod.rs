use api_core::{
    api::{CoreError, MutateCategories},
    Category,
};
use tracing::{debug, instrument};

use crate::Client;

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

    async fn update_category(
        &self,
        id: impl AsRef<str> + Send + Sync,
        data: &Category,
    ) -> Result<Category, CoreError> {
        todo!()
    }

    async fn delete_category(
        &self,
        id: impl AsRef<str> + Send + Sync,
    ) -> Result<Category, CoreError> {
        todo!()
    }
}

#[derive(serde::Serialize)]
struct InputCategory<'a> {
    name: &'a str,
    sub_categories: &'a [String],
    image_url: Option<&'a str>,
    is_root: bool,
}

impl<'a> From<&'a Category> for InputCategory<'a> {
    fn from(value: &'a Category) -> Self {
        Self {
            name: &value.name,
            sub_categories: &value.sub_categories,
            image_url: value.image_url.as_deref(),
            is_root: value.is_root,
        }
    }
}
