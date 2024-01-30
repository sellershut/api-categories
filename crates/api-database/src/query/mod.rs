use std::fmt::Debug;

use api_core::{
    api::{CoreError, QueryCategories},
    Category,
};
use surrealdb::sql::Thing;
use tracing::{error, instrument};

use crate::{
    collections::Collections,
    redis::{cache_keys::CacheKey, redis_query},
    Client,
};

impl QueryCategories for Client {
    #[instrument(skip(self), err(Debug))]
    async fn get_categories(&self) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        if let Some(ref redis) = self.redis {
            let cache_key = CacheKey::AllCategories;
            let categories = redis_query::query::<Vec<Category>>(cache_key, redis).await;

            if let Some(categories) = categories {
                Ok(categories.into_iter())
            } else {
                let categories: Vec<Category> = self.client.select(Collections::Category).await?;

                if let Err(e) =
                    redis_query::update(cache_key, redis, &categories, self.cache_ttl).await
                {
                    error!(key = %cache_key, "[redis update]: {e}");
                }

                Ok(categories.into_iter())
            }
        } else {
            let categories: Vec<Category> = self.client.select(Collections::Category).await?;
            Ok(categories.into_iter())
        }
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_sub_categories(
        &self,
        id: Option<impl AsRef<str> + Send + Debug>,
    ) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        let id = id.as_ref().map(|id| id.as_ref());
        let caller = |id: Option<&str>| {
            self.client.query(if let Some(parent) = id {
                format!("SELECT sub_categories.*.* FROM {};", parent)
            } else {
                format!("SELECT * FROM {} WHERE is_root=true", Collections::Category)
            })
        };
        if let Some(ref redis) = self.redis {
            let cache_key = CacheKey::SubCategories { parent: id };

            let categories = redis_query::query::<Vec<Category>>(cache_key, redis).await;
            if let Some(categories) = categories {
                Ok(categories.into_iter())
            } else {
                let mut resp = caller(id).await?;
                let categories: Vec<Category> = resp.take(0)?;

                if let Err(e) =
                    redis_query::update(cache_key, redis, &categories, self.cache_ttl).await
                {
                    error!(key = %cache_key, "[redis update]: {e}");
                }
                Ok(categories.into_iter())
            }
        } else {
            let mut resp = caller(id).await?;
            let categories: Vec<Category> = resp.take(0)?;

            Ok(categories.into_iter())
        }
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_category_by_id(
        &self,
        id: impl AsRef<str> + Send + Debug,
    ) -> Result<Option<Category>, CoreError> {
        let create_id =
            |id: &str| -> Thing { Thing::from((Collections::Category.to_string().as_str(), id)) };

        let id = id.as_ref();
        if id.is_empty() {
            return Err(CoreError::Other("Id cannot be empty".into()));
        }

        if let Some(ref redis) = self.redis {
            let cache_key = CacheKey::Category { id };

            let category = redis_query::query::<Category>(cache_key, redis).await;

            if category.is_some() {
                Ok(category)
            } else {
                let id = create_id(id);

                let category = self.client.select(id).await?;

                if let Err(e) =
                    redis_query::update(cache_key, redis, category.as_ref(), self.cache_ttl).await
                {
                    error!(key = %cache_key, "[redis update]: {e}");
                }
                Ok(category)
            }
        } else {
            let id = create_id(id);

            let category = self.client.select(id).await?;

            Ok(category)
        }
    }
}
