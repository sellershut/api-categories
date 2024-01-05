use crate::graphql::QueryRoot;
use api_database::Client;
use async_graphql::{
    extensions::{OpenTelemetry, Tracing},
    EmptySubscription, Schema,
};
use opentelemetry::trace::Tracer;

use self::graphql::mutation::Mutation;

pub mod graphql;

pub async fn create_schema<T>(tracer: T) -> Schema<QueryRoot, Mutation, EmptySubscription>
where
    T: Tracer + Send + Sync + 'static,
    <T as Tracer>::Span: Sync + Send,
{
    let db_client = Client::try_new("0.0.0.0:8000", "root", "root")
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
