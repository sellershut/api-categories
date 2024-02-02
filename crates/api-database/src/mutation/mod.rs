use std::fmt::Debug;

use api_core::{
    api::{CoreError, MutateCategories},
    Category,
};
use surrealdb::{opt::RecordId, sql::Thing};
use tracing::instrument;
use uuid::Uuid;

use crate::{collections::Collections, entity::DatabaseEntity, Client};

impl MutateCategories for Client {
    #[instrument(skip(self), err(Debug))]
    async fn create_category(&self, category: &Category) -> Result<Category, CoreError> {
        let input_category = InputCategory::from(category);

        let id = Uuid::now_v7().to_string();
        let item: Option<DatabaseEntity> = self
            .client
            .create(("category", id))
            .content(input_category)
            .await
            .unwrap();

        match item {
            Some(e) => Category::try_from(e),
            None => Err(CoreError::Unreachable),
        }
    }

    #[instrument(skip(self, id), err(Debug))]
    async fn update_category(
        &self,
        id: impl AsRef<str> + Send + Debug,
        data: &Category,
    ) -> Result<Option<Category>, CoreError> {
        let id = id.as_ref();
        let id = Thing::from((Collections::Category.to_string().as_str(), id));

        let input_category = InputCategory::from(data);

        let item: Option<DatabaseEntity> = self
            .client
            .update(id)
            .content(input_category)
            .await
            .unwrap();
        let res = match item {
            Some(e) => Some(Category::try_from(e)?),
            None => None,
        };

        Ok(res)
    }

    #[instrument(skip(self, id), err(Debug))]
    async fn delete_category(
        &self,
        id: impl AsRef<str> + Send + Debug,
    ) -> Result<Option<Category>, CoreError> {
        let id = Thing::from((Collections::Category.to_string().as_str(), id.as_ref()));

        let res: Option<DatabaseEntity> = self.client.delete(id).await.unwrap();
        let res = match res {
            Some(e) => Some(Category::try_from(e)?),
            None => None,
        };

        Ok(res)
    }
}

#[derive(serde::Serialize)]
struct InputCategory<'a> {
    name: &'a str,
    sub_categories: Option<Vec<RecordId>>,
    image_url: Option<&'a str>,
    is_root: bool,
}

impl<'a> From<&'a Category> for InputCategory<'a> {
    fn from(value: &'a Category) -> Self {
        Self {
            name: &value.name,
            sub_categories: value.sub_categories.as_ref().map(|f| {
                f.iter()
                    .map(|str| RecordId::from(("category", str.to_string().as_str())))
                    .collect()
            }),
            image_url: value.image_url.as_deref(),
            is_root: value.is_root,
        }
    }
}
