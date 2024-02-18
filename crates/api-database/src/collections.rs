use surrealdb::{
    opt::{IntoResource, Resource},
    sql::Table,
};

#[non_exhaustive]
pub(crate) enum Collections {
    Category,
}

impl std::fmt::Display for Collections {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Collections::Category => "category",
            }
        )
    }
}

impl<R> IntoResource<Vec<R>> for Collections {
    fn into_resource(self) -> Result<Resource, surrealdb::Error> {
        Ok(Resource::Table(Table(self.to_string())))
    }
}
