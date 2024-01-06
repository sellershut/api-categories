pub mod api;

#[cfg(feature = "async-graphql")]
use async_graphql::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "async-graphql", derive(InputObject, SimpleObject))]
#[cfg_attr(feature = "async-graphql", graphql(input_name = "CategoryInput"))]
pub struct Category {
    #[cfg_attr(feature = "async-graphql", graphql(skip_input), graphql(flatten))]
    pub id: Id,
    pub name: String,
    pub sub_categories: Vec<String>,
    pub image_url: Option<String>,
    pub is_root: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Id {
    String(String),
    #[cfg(feature = "surrealdb")]
    Thing(surrealdb::sql::Thing),
}

#[cfg(feature = "async-graphql")]
#[cfg_attr(feature = "async-graphql", Object)]
impl Id {
    async fn id(&self) -> String {
        match self {
            Id::String(e) => e.to_owned(),
            #[cfg(feature = "surrealdb")]
            Id::Thing(e) => e.id.to_raw(),
        }
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
