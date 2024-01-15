use std::sync::Arc;

use async_graphql::{extensions::{ExtensionFactory, Extension, ExtensionContext, NextParseQuery, NextExecute}, Variables, ServerResult, parser::types::ExecutableDocument, Response};
use axum::async_trait;

pub struct Metrics;

impl ExtensionFactory for Metrics{
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(MetricsExtension)
    }
}

struct MetricsExtension;

#[async_trait]
impl Extension for MetricsExtension {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        todo!()
    }

    async fn execute(
        &self,
        ctx: &ExtensionContext<'_>,
        operation_name: Option<&str>,
        next: NextExecute<'_>,
    ) -> Response {
        todo!()
    }
}
