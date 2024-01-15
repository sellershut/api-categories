use api_database::Client;
use async_graphql::{extensions::ExtensionFactory, Schema, SchemaBuilder};

use self::graphql::{mutation::Mutation, query::Query, subscription::Subscription};

pub mod graphql;

#[derive(Debug, Clone, Copy)]
pub struct DatabaseCredentials<'a> {
    pub db_dsn: &'a str,
    pub db_user: &'a str,
    pub db_pass: &'a str,
    pub db_ns: &'a str,
    pub db: &'a str,
}

pub struct ApiSchemaBuilder {
    builder: SchemaBuilder<Query, Mutation, Subscription>,
}

impl ApiSchemaBuilder {
    pub async fn new(database: DatabaseCredentials<'_>) -> Self {
        let db_client = Client::try_new(
            database.db_dsn,
            database.db_user,
            database.db_pass,
            database.db_ns,
            database.db,
        )
        .await
        .unwrap();
        Self {
            builder: Schema::build(
                Query::default(),
                Mutation::default(),
                Subscription::default(),
            )
            .data(db_client),
        }
    }

    pub fn with_extension(self, extension: impl ExtensionFactory) -> Self {
        Self {
            builder: self.builder.extension(extension),
        }
    }

    pub fn build(self) -> Schema<Query, Mutation, Subscription> {
        self.builder.finish()
    }
}

#[cfg(test)]
mod tests;
