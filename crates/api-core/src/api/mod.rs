mod error;
pub use std::fmt::Debug;

use crate::Category;

pub use error::*;

#[trait_variant::make(QueryCategories: Send)]
pub trait LocalQueryCategories {
    async fn get_categories(&self) -> Result<impl ExactSizeIterator<Item = Category>, CoreError>;
    async fn get_sub_categories(
        &self,
        id: Option<impl AsRef<str> + Send + Debug>,
    ) -> Result<impl ExactSizeIterator<Item = Category>, CoreError>;
    async fn get_category_by_id(
        &self,
        id: impl AsRef<str> + Send + Debug,
    ) -> Result<Option<Category>, CoreError>;
}

#[trait_variant::make(MutateCategories: Send)]
pub trait LocalMutateCategories {
    async fn create_category(&self, category: &Category) -> Result<Category, CoreError>;
    async fn update_category(
        &self,
        id: impl AsRef<str> + Send + Debug,
        data: &Category,
    ) -> Result<Option<Category>, CoreError>;
    async fn delete_category(
        &self,
        id: impl AsRef<str> + Send + Debug,
    ) -> Result<Option<Category>, CoreError>;
}
