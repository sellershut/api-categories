use api_core::{
    api::{CoreError, QueryCategories},
    reexports::uuid::Uuid,
    Category,
};
use meilisearch_sdk::{SearchQuery, SearchResults};
use surrealdb::sql::Thing;
use tracing::{debug, error, instrument};

use crate::{
    collections::Collection,
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
                .query("SELECT sub_categories.*.* FROM type::thing($record)")
                .bind((
                    "record",
                    Thing::from((Collection::Category.to_string(), id.to_string())),
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
                .query("SELECT * FROM type::table($table) WHERE parent_id is none or null")
                .bind(("table", Collection::Category))
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

async fn db_get_categories(
    db: &Client,
    wait_for_completion: bool,
) -> Result<std::vec::IntoIter<Category>, CoreError> {
    let categories = if let Some((ref redis, _ttl)) = db.redis {
        let cache_key = CacheKey::AllCategories;
        let categories = redis_query::query::<Vec<Category>>(cache_key, redis).await;

        if let Some(categories) = categories {
            categories
        } else {
            let categories: Vec<DatabaseEntity> = db
                .client
                .select(Collection::Category)
                .await
                .map_err(map_db_error)?;

            let categories = categories
                .into_iter()
                .map(Category::try_from)
                .collect::<Result<Vec<Category>, CoreError>>()?;

            if let Err(e) = redis_query::update(cache_key, redis, &categories, None).await {
                error!(key = %cache_key, "[redis update]: {e}");
            }

            categories
        }
    } else {
        let categories: Vec<DatabaseEntity> = db
            .client
            .select(Collection::Category)
            .await
            .map_err(map_db_error)?;
        categories
            .into_iter()
            .map(Category::try_from)
            .collect::<Result<Vec<Category>, CoreError>>()?
    };

    if let Some(ref client) = db.search_client {
        debug!("indexing categories for search");
        let index = client.index("categories");
        let task = index
            .add_documents(&categories, Some("id"))
            .await
            .map_err(|e| CoreError::Other(e.to_string()))?;

        if wait_for_completion {
            if let Err(e) = task.wait_for_completion(client, None, None).await {
                error!("{e}");
            }
        }
    }

    Ok(categories.into_iter())
}

impl QueryCategories for Client {
    #[instrument(skip(self), err(Debug))]
    async fn get_categories(&self) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        db_get_categories(self, false).await
    }

    #[instrument(skip(self), err(Debug))]
    async fn get_sub_categories(
        &self,
        id: Option<&Uuid>,
    ) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        if let Some((ref redis, _ttl)) = self.redis {
            let cache_key = CacheKey::SubCategories { parent: id };

            let categories = redis_query::query::<Vec<Category>>(cache_key, redis).await;
            if let Some(categories) = categories {
                Ok(categories.into_iter())
            } else {
                let categories = db_get_sub_categories(self, id).await?;

                if let Err(e) = redis_query::update(cache_key, redis, &categories, None).await {
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
                Collection::Category.to_string().as_str(),
                id.to_string().as_str(),
            ))
        };

        if let Some((ref redis, _)) = self.redis {
            let cache_key = CacheKey::Category { id };

            let category = redis_query::query::<Option<Category>>(cache_key, redis).await;

            if let Some(category) = category {
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

                if let Err(e) =
                    // set ttl to 5 mins
                    redis_query::update(cache_key, redis, category.as_ref(), None).await
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
            let mut index = None;
            for _retries in 0..3 {
                if let Ok(idx) = client.get_index("categories").await {
                    index = Some(idx);
                    break;
                }
                let _categories = db_get_categories(self, true).await?;
            }
            match index {
                Some(index) => {
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
                }
                None => Err(CoreError::Other(
                    "items could not be indexed for search".into(),
                )),
            }
        } else {
            Err(CoreError::Other(String::from(
                "no client configured for search",
            )))
        }
    }
}

impl Client {
    pub async fn search_with_parent_name(
        &self,
        query: &str,
    ) -> Result<Vec<(Category, Option<String>)>, CoreError> {
        if let Some(ref client) = self.search_client {
            let mut index = None;
            for _retries in 0..3 {
                if let Ok(idx) = client.get_index("categories").await {
                    index = Some(idx);
                    break;
                }
                let _categories = db_get_categories(self, true).await?;
            }
            match index {
                Some(index) => {
                    let query = SearchQuery::new(&index).with_query(query.as_ref()).build();

                    let results: SearchResults<Category> = index
                        .execute_query(&query)
                        .await
                        .map_err(|e| CoreError::Other(e.to_string()))?;

                    let parent_ids: Vec<_> = results
                        .hits
                        .iter()
                        .filter_map(|f| f.result.parent_id.map(|parent_id| parent_id.to_string()))
                        .collect();
                    let parent_ids_str: Vec<&str> = parent_ids.iter().map(|f| f.as_str()).collect();

                    let futures = parent_ids_str
                        .iter()
                        .map(|parent_id| index.get_document::<Category>(parent_id));

                    let res: Vec<Category> = futures_util::future::try_join_all(futures)
                        .await
                        .map_err(|e| CoreError::Other(e.to_string()))?;

                    let search_results: Vec<_> = results
                        .hits
                        .into_iter()
                        .map(|hit| {
                            let category = Category {
                                id: hit.result.id,
                                name: hit.result.name,
                                sub_categories: hit.result.sub_categories,
                                parent_id: hit.result.parent_id,
                                image_url: hit.result.image_url,
                            };
                            let parent = if let Some(parent_id) = hit.result.parent_id {
                                res.iter().find_map(|category| {
                                    if parent_id == category.id {
                                        Some(category.name.to_owned())
                                    } else {
                                        None
                                    }
                                })
                            } else {
                                None
                            };
                            (category, parent)
                        })
                        .collect();

                    Ok(search_results)
                }
                None => Err(CoreError::Other(
                    "items could not be indexed for search".into(),
                )),
            }
        } else {
            Err(CoreError::Other(String::from(
                "no client configured for search",
            )))
        }
    }
}
