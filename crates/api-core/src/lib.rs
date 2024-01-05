pub mod api;

#[cfg(feature = "async-graphql")]
use async_graphql::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "async-graphql", derive(InputObject, SimpleObject))]
pub struct Category {
    #[cfg(all(feature = "surrealdb", not(feature = "async-graphql")))]
    id: surrealdb::sql::Thing,
    #[cfg(all(feature = "surrealdb", feature = "async-graphql"))]
    #[cfg_attr(
        all(feature = "async-graphql", feature = "surrealdb"),
        graphql(skip_input)
    )]
    id: Id,
    #[cfg(all(not(feature = "surrealdb"), feature = "async-graphql"))]
    #[cfg_attr(
        all(feature = "async-graphql", not(feature = "surrealdb")),
        graphql(skip_input)
    )]
    id: String,
    name: String,
    sub_categories: Vec<String>,
    image_url: Option<String>,
    is_root: bool,
}

#[cfg(all(feature = "surrealdb", feature = "async-graphql"))]
#[cfg_attr(
    all(feature = "async-graphql", feature = "surrealdb"),
    derive(Clone, Eq, Debug, PartialEq, PartialOrd, Ord)
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Id(surrealdb::sql::Thing);

#[cfg(all(feature = "surrealdb", feature = "async-graphql"))]
#[cfg_attr(all(feature = "async-graphql", feature = "surrealdb"), Object)]
impl Id {
    async fn value(&self) -> String {
        self.0.to_raw()
    }
}

#[cfg(all(feature = "surrealdb", feature = "async-graphql"))]
impl Default for Id {
    fn default() -> Self {
        Self(<surrealdb::sql::Thing as std::str::FromStr>::from_str("default:thing").expect("creating default thing"))
    }
}
