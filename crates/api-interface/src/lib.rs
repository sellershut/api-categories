use crate::graphql::QueryRoot;
use api_database::Client;
use async_graphql::{
    extensions::{OpenTelemetry, Tracing},
    EmptySubscription, Schema,
};
use opentelemetry::trace::Tracer;

use self::graphql::mutation::Mutation;

pub mod graphql;

#[derive(Debug, Clone, Copy)]
pub struct DatabaseCredentials<'a> {
    pub db_dsn: &'a str,
    pub db_user: &'a str,
    pub db_pass: &'a str,
    pub db_ns: &'a str,
    pub db: &'a str,
}

pub async fn create_schema<T>(
    tracer: T,
    database: DatabaseCredentials<'_>,
) -> Schema<QueryRoot, Mutation, EmptySubscription>
where
    T: Tracer + Send + Sync + 'static,
    <T as Tracer>::Span: Sync + Send,
{
    let db_client = Client::try_new(
        database.db_dsn,
        database.db_user,
        database.db_pass,
        database.db_ns,
        database.db,
    )
    .await
    .unwrap();
    Schema::build(QueryRoot, Mutation::default(), EmptySubscription)
        .data(db_client)
        .extension(Tracing)
        .extension(OpenTelemetry::new(tracer))
        .finish()
}

#[cfg(test)]
mod tests;
