use api_core::{
    api::{CoreError, QueryCategories},
    reexports::uuid::Uuid,
    Category,
};
use surrealdb::sql::Thing;
use tracing::{error, instrument};

use crate::{
    collections::Collections,
    entity::DatabaseEntity,
    redis::{cache_keys::CacheKey, redis_query},
    Client,
};

impl QueryCategories for Client {
    #[instrument(skip(self), err(Debug))]
    async fn get_categories(&self) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        if let Some((ref redis, ttl)) = self.redis {
            let cache_key = CacheKey::AllCategories;
            let categories = redis_query::query::<Vec<Category>>(cache_key, redis).await;

            if let Some(categories) = categories {
                Ok(categories.into_iter())
            } else {
                let categories: Vec<DatabaseEntity> =
                    self.client.select(Collections::Category).await.unwrap();

                let categories = categories
                    .into_iter()
                    .map(Category::try_from)
                    .collect::<Result<Vec<Category>, CoreError>>()?;

                if let Err(e) = redis_query::update(cache_key, redis, &categories, ttl).await {
                    error!(key = %cache_key, "[redis update]: {e}");
                }

                Ok(categories.into_iter())
            }
        } else {
            let categories: Vec<DatabaseEntity> =
                self.client.select(Collections::Category).await.unwrap();
            let categories = categories
                .into_iter()
                .map(Category::try_from)
                .collect::<Result<Vec<Category>, CoreError>>()?;
            Ok(categories.into_iter())
        }
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_sub_categories(
        &self,
        id: Option<&Uuid>,
    ) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        let caller = |id: Option<&Uuid>| {
            self.client.query(if let Some(parent) = id {
                format!("SELECT sub_categories.*.* FROM {};", parent)
            } else {
                format!(
                    "SELECT * FROM {} WHERE parent_id IS NOT NULL",
                    Collections::Category
                )
            })
        };
        if let Some((ref redis, ttl)) = self.redis {
            let cache_key = CacheKey::SubCategories { parent: id };

            let categories = redis_query::query::<Vec<Category>>(cache_key, redis).await;
            if let Some(categories) = categories {
                Ok(categories.into_iter())
            } else {
                let mut resp = caller(id).await.unwrap();
                let categories: Vec<DatabaseEntity> = resp.take(0).unwrap();
                let categories = categories
                    .into_iter()
                    .map(Category::try_from)
                    .collect::<Result<Vec<Category>, CoreError>>()?;

                if let Err(e) = redis_query::update(cache_key, redis, &categories, ttl).await {
                    error!(key = %cache_key, "[redis update]: {e}");
                }
                Ok(categories.into_iter())
            }
        } else {
            let mut resp = caller(id).await.unwrap();
            let categories: Vec<DatabaseEntity> = resp.take(0).unwrap();
            let categories = categories
                .into_iter()
                .map(Category::try_from)
                .collect::<Result<Vec<Category>, CoreError>>()?;

            Ok(categories.into_iter())
        }
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_category_by_id(&self, id: &Uuid) -> Result<Option<Category>, CoreError> {
        let create_id = |id: &Uuid| -> Thing {
            Thing::from((
                Collections::Category.to_string().as_str(),
                id.to_string().as_str(),
            ))
        };

        if let Some((ref redis, ttl)) = self.redis {
            let cache_key = CacheKey::Category { id };

            let category = redis_query::query::<Category>(cache_key, redis).await;

            if category.is_some() {
                Ok(category)
            } else {
                let id = create_id(id);

                let category: Option<DatabaseEntity> = self.client.select(id).await.unwrap();
                let category = category.and_then(|f| match Category::try_from(f) {
                    Ok(cat) => Some(cat),
                    Err(e) => {
                        error!("{e}");
                        None
                    }
                });

                if let Err(e) = redis_query::update(cache_key, redis, category.as_ref(), ttl).await
                {
                    error!(key = %cache_key, "[redis update]: {e}");
                }
                Ok(category)
            }
        } else {
            let id = create_id(id);

            let category: Option<DatabaseEntity> = self.client.select(id).await.unwrap();
            let category = category.and_then(|f| match Category::try_from(f) {
                Ok(cat) => Some(cat),
                Err(e) => {
                    error!("{e}");
                    None
                }
            });

            Ok(category)
        }
    }
}
