use std::str::FromStr;

use thiserror::Error;

use crate::Category;

#[trait_variant::make(QueryCategories: Send)]
pub trait LocalQueryCategories {
    async fn get_categories(&self) -> Result<impl ExactSizeIterator<Item = Category>, CoreError>;
    async fn get_sub_categories(
        &self,
        id: impl AsRef<str>,
    ) -> Result<impl ExactSizeIterator<Item = Category>, CoreError>;
    async fn get_category_by_id(&self, id: impl AsRef<str>) -> Result<Option<Category>, CoreError>;
}

#[trait_variant::make(MutateCategories: Send)]
pub trait LocalMutateCategories {
    async fn create_category(&self, category: &Category) -> Result<Category, CoreError>;
    async fn update_category(
        &self,
        id: impl AsRef<str>,
        data: &Category,
    ) -> Result<Category, CoreError>;
    async fn delete_category(&self, id: impl AsRef<str>) -> Result<Category, CoreError>;
}

#[derive(Error, Debug)]
pub enum CoreError {
    #[cfg(feature = "surrealdb")]
    #[error("data store disconnected")]
    Database(#[from] surrealdb::Error),
    #[error("`{0}`")]
    Other(String),
    #[error("unknown core error")]
    Unknown,
}

impl FromStr for CoreError {
    type Err = Self;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Other(s.to_owned()))
    }
}
