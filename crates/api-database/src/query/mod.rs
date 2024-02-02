use std::fmt::Debug;

use api_core::{
    api::{CoreError, QueryCategories},
    Category,
};
use meilisearch_sdk::{SearchQuery, SearchResults};
use surrealdb::sql::Thing;
use tracing::{debug, error, instrument};

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
                let categories: Vec<DatabaseEntity> = self
                    .client
                    .select(Collections::Category)
                    .await
                    .map_err(|e| CoreError::Database(e.to_string()))?;

                let categories = categories
                    .into_iter()
                    .map(Category::try_from)
                    .collect::<Result<Vec<Category>, CoreError>>()?;

                if let Some(ref client) = self.search_client {
                    debug!("indexing categories for search");
                    let index = client.index("categories");
                    index
                        .add_documents(&categories, Some("id"))
                        .await
                        .map_err(|e| CoreError::Other(e.to_string()))?;
                }

                if let Err(e) = redis_query::update(cache_key, redis, &categories, ttl).await {
                    error!(key = %cache_key, "[redis update]: {e}");
                }

                Ok(categories.into_iter())
            }
        } else {
            let categories: Vec<DatabaseEntity> =
                self.client
                    .select(Collections::Category)
                    .await
                    .map_err(|e| CoreError::Database(e.to_string()))?;
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
        if let Some((ref redis, ttl)) = self.redis {
            let cache_key = CacheKey::SubCategories { parent: id };

            let categories = redis_query::query::<Vec<Category>>(cache_key, redis).await;
            if let Some(categories) = categories {
                Ok(categories.into_iter())
            } else {
                let mut resp = caller(id)
                    .await
                    .map_err(|e| CoreError::Database(e.to_string()))?;
                let categories: Vec<DatabaseEntity> = resp
                    .take(0)
                    .map_err(|e| CoreError::Database(e.to_string()))?;
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
            let mut resp = caller(id)
                .await
                .map_err(|e| CoreError::Database(e.to_string()))?;
            let categories: Vec<DatabaseEntity> = resp
                .take(0)
                .map_err(|e| CoreError::Database(e.to_string()))?;
            let categories = categories
                .into_iter()
                .map(Category::try_from)
                .collect::<Result<Vec<Category>, CoreError>>()?;

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

        if let Some((ref redis, ttl)) = self.redis {
            let cache_key = CacheKey::Category { id };

            let category = redis_query::query::<Category>(cache_key, redis).await;

            if category.is_some() {
                Ok(category)
            } else {
                let id = create_id(id);

                let category: Option<DatabaseEntity> = self
                    .client
                    .select(id)
                    .await
                    .map_err(|e| CoreError::Database(e.to_string()))?;
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

            let category: Option<DatabaseEntity> = self
                .client
                .select(id)
                .await
                .map_err(|e| CoreError::Database(e.to_string()))?;
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

    async fn search(
        &self,
        query: impl AsRef<str> + Send + Debug,
    ) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        if let Some(ref client) = self.search_client {
            let index = client
                .get_index("categories")
                .await
                .map_err(|e| CoreError::Other(e.to_string()))?;

            let query = SearchQuery::new(&index).with_query(query.as_ref()).build();

            let results: SearchResults<Category> = index
                .execute_query(&query)
                .await
                .map_err(|e| CoreError::Database(e.to_string()))?;

            let search_results: Vec<Category> = results
                .hits
                .into_iter()
                .map(|hit| Category {
                    id: hit.result.id,
                    name: hit.result.name,
                    sub_categories: hit.result.sub_categories,
                    is_root: hit.result.is_root,
                    image_url: hit.result.image_url,
                })
                .collect();

            Ok(search_results.into_iter())
        } else {
            Err(CoreError::Other(String::from(
                "no client configured for search",
            )))
        }
    }
}
