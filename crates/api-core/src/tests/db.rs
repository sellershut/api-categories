use std::{fmt::Debug, str::FromStr};

use crate::{
    api::{
        CoreError, LocalMutateCategories, LocalQueryCategories, MutateCategories, QueryCategories,
    },
    Category,
};

pub struct SampleDb;
pub struct SampleDbSend;

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

impl LocalMutateCategories for SampleDb {
    async fn create_category(&self, category: &Category) -> Result<Category, CoreError> {
        Ok(category.to_owned())
    }

    async fn update_category(
        &self,
        id: impl AsRef<str> + Send + Debug,
        data: &Category,
    ) -> Result<Option<Category>, CoreError> {
        if id.as_ref().is_empty() {
            Err(CoreError::from_str("Id cannot be empty")?)
        } else {
            Ok(Some(data.to_owned()))
        }
    }

    async fn delete_category(
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

impl MutateCategories for SampleDbSend {
    async fn create_category(&self, category: &Category) -> Result<Category, CoreError> {
        Ok(category.to_owned())
    }

    async fn update_category(
        &self,
        id: impl AsRef<str> + Send + Debug,
        data: &Category,
    ) -> Result<Option<Category>, CoreError> {
        if id.as_ref().is_empty() {
            Err(CoreError::from_str("Id cannot be empty")?)
        } else {
            Ok(Some(data.to_owned()))
        }
    }

    async fn delete_category(
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

impl QueryCategories for SampleDbSend {
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
