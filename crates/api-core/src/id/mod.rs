#[cfg(feature = "async-graphql")]
mod input;

#[cfg(feature = "async-graphql")]
mod output;

use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
#[cfg_attr(
    all(feature = "serde", feature = "surrealdb"),
    serde(from = "surrealdb::sql::Thing")
)]
#[cfg_attr(
    all(feature = "serde", not(feature = "surrealdb")),
    serde(from = "String")
)]
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Id {
    #[cfg(not(feature = "surrealdb"))]
    String(String),
    #[cfg(feature = "surrealdb")]
    Thing(surrealdb::sql::Thing),
}

#[cfg(feature = "surrealdb")]
impl From<surrealdb::sql::Thing> for Id {
    fn from(value: surrealdb::sql::Thing) -> Self {
        Id::Thing(value)
    }
}

#[cfg(not(feature = "surrealdb"))]
impl From<String> for Id {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl std::str::FromStr for Id {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(feature = "surrealdb")]
        {
            let thing = surrealdb::sql::Thing::from_str(s)?;
            Ok(Self::Thing(thing))
        }

        #[cfg(not(feature = "surrealdb"))]
        {
            Ok(Self::String(s.to_owned()))
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                #[cfg(not(feature = "surrealdb"))]
                Id::String(e) => e.to_owned(),
                #[cfg(feature = "surrealdb")]
                Id::Thing(e) => e.id.to_raw(),
            }
        )
    }
}

impl Default for Id {
    fn default() -> Self {
        #[cfg(not(feature = "surrealdb"))]
        return Self::String(String::default());
        #[cfg(feature = "surrealdb")]
        return Self::Thing(surrealdb::sql::Thing::from(("category", "default")));
    }
}
