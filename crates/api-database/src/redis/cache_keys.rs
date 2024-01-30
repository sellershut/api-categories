use std::fmt::Display;

use redis::ToRedisArgs;

#[derive(Clone, Copy)]
pub enum CacheKey<'a> {
    AllCategories,
    SubCategories { parent: Option<&'a str> },
    Category { id: &'a str },
}

impl Display for CacheKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "categories:{}",
            match self {
                CacheKey::AllCategories => "all".to_string(),
                CacheKey::SubCategories { parent } => format!(
                    "parent={}",
                    match parent {
                        Some(id) => id,
                        None => {
                            ""
                        }
                    }
                ),
                CacheKey::Category { id } => format!("id={id}"),
            }
        )
    }
}

impl ToRedisArgs for CacheKey<'_> {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(self.to_string().as_bytes())
    }
}
