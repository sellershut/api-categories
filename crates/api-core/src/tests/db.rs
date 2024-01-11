use std::{fmt::Debug, str::FromStr};

use crate::{
    api::{CoreError, LocalQueryCategories},
    Category,
};

pub struct SampleDb;

impl LocalQueryCategories for SampleDb {
    async fn get_categories(&self) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_sub_categories(
        &self,
        _id: Option<impl AsRef<str> + Send + Debug>,
    ) -> Result<impl ExactSizeIterator<Item = Category>, CoreError> {
        Ok([].into_iter())
    }

    async fn get_category_by_id(
        &self,
        id: impl AsRef<str> + Send + Debug,
    ) -> Result<Option<Category>, CoreError> {
        if id.as_ref().is_empty() {
            Err(CoreError::from_str("Id cannot be empty")?)
        } else {
            Ok(None)
        }
    }
}
