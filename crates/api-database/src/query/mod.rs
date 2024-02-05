use api_core::{
    api::{CoreError, QueryCategories},
    reexports::uuid::Uuid,
    Category,
};
use meilisearch_sdk::{SearchQuery, SearchResults};
use surrealdb::sql::Thing;
use tracing::{debug, error, instrument};

use crate::{
    collections::Collections,
    entity::DatabaseEntity,
    map_db_error,
    redis::{cache_keys::CacheKey, redis_query},
    Client,
};

async fn db_get_sub_categories(db: &Client, id: Option<&Uuid>) -> Result<Vec<Category>, CoreError> {
    match id {
        Some(id) => {
            let mut res = db
                .client
                .query(format!(
                    "SELECT sub_categories.*.* FROM {}:⟨{}⟩",
                    Collections::Category,
                    id
                ))
                .await
                .map_err(map_db_error)?;

            let res: Option<Vec<DatabaseEntity>> =
                res.take((0, "sub_categories")).map_err(map_db_error)?;

            let categories = res
                .ok_or(CoreError::Database("Database returned no items".into()))
                .and_then(|vals: Vec<DatabaseEntity>| {
                    vals.into_iter()
                        .map(Category::try_from)
                        .collect::<Result<Vec<Category>, CoreError>>()
                })?;
            Ok(categories)
        }
        None => {
            let mut resp = db
                .client
                .query(format!(
                    "SELECT * FROM {} WHERE parent_id is none or null",
                    Collections::Category
                ))
                .await
                .map_err(map_db_error)?;
            let categories: Vec<DatabaseEntity> = resp.take(0).map_err(map_db_error)?;
            let categories = categories
                .into_iter()
                .map(Category::try_from)
                .collect::<Result<Vec<Category>, CoreError>>()?;
            Ok(categories)
        }
    }
}

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
                    .map_err(map_db_error)?;

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
            let categories: Vec<DatabaseEntity> = self
                .client
                .select(Collections::Category)
                .await
                .map_err(map_db_error)?;
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
        if let Some((ref redis, ttl)) = self.redis {
            let cache_key = CacheKey::SubCategories { parent: id };

            let categories = redis_query::query::<Vec<Category>>(cache_key, redis).await;
            if let Some(categories) = categories {
                Ok(categories.into_iter())
            } else {
                let categories = db_get_sub_categories(self, id).await?;

                if let Err(e) = redis_query::update(cache_key, redis, &categories, ttl).await {
                    error!(key = %cache_key, "[redis update]: {e}");
                }
                Ok(categories.into_iter())
            }
        } else {
            let categories = db_get_sub_categories(self, id).await?;

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

                let category: Option<DatabaseEntity> =
                    self.client.select(id).await.map_err(map_db_error)?;
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

            let category: Option<DatabaseEntity> =
                self.client.select(id).await.map_err(map_db_error)?;
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
        query: impl AsRef<str> + Send + std::fmt::Debug,
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
                .map_err(|e| CoreError::Other(e.to_string()))?;

            let search_results: Vec<Category> = results
                .hits
                .into_iter()
                .map(|hit| Category {
                    id: hit.result.id,
                    name: hit.result.name,
                    sub_categories: hit.result.sub_categories,
                    parent_id: hit.result.parent_id,
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
