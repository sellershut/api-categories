#[cfg(feature = "async-graphql")]
mod input;

#[cfg(feature = "async-graphql")]
mod output;

use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Id {
    String(String),
    #[cfg(feature = "surrealdb")]
    Thing(surrealdb::sql::Thing),
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
                Id::String(e) => e.to_owned(),
                #[cfg(feature = "surrealdb")]
                Id::Thing(e) => e.id.to_raw(),
            }
        )
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::String(String::default())
    }
}
