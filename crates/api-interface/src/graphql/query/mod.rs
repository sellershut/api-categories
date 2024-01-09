use async_graphql::connection::{Connection, EmptyFields};

pub(crate) mod category;
pub(crate) mod pagination;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(category::CategoryQuery);

pub(crate) type ConnectionResult<T> = async_graphql::Result<
    Connection<pagination::Base64Cursor, T, pagination::ConnectionFields, EmptyFields>,
>;

/// Relay-compliant connection parameters to page results by cursor/page size
pub struct Params {
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
}

impl Params {
    pub const fn new(
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Self {
        Self {
            after,
            before,
            first,
            last,
        }
    }
}
