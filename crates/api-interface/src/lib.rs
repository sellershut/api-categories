use opentelemetry::trace::Tracer;
use crate::graphql::QueryRoot;
use async_graphql::{
    extensions::{OpenTelemetry, Tracing},
    EmptyMutation, EmptySubscription, Schema,
};

pub mod graphql;

pub fn create_schema<T>(tracer: T) -> Schema<QueryRoot, EmptyMutation, EmptySubscription>
where
    T: Tracer + Send + Sync + 'static,
    <T as Tracer>::Span: Sync + Send,
{
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(graphql::StarWars::new())
        .extension(Tracing)
        .extension(OpenTelemetry::new(tracer))
        .finish()
}

#[cfg(test)]
mod tests;
