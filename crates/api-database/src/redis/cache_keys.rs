/* use std::fmt::Display;

pub enum CacheKeys<'a> {
    AllCategories,
    SubCategories { parent: &'a str },
    Category { id: &'a str },
}

impl Display for CacheKeys<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "categories:{}",
            match self {
                CacheKeys::AllCategories => "all".to_string(),
                CacheKeys::SubCategories { parent } => format!("parent={parent}"),
                CacheKeys::Category { id } => format!("id={id}"),
            }
        )
    }
} */
