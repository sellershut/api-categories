use std::sync::Arc;

use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextPrepareRequest},
    Request, ServerResult,
};
use axum::async_trait;

pub struct Metrics;

impl ExtensionFactory for Metrics {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(MetricsExtension)
    }
}

struct MetricsExtension;

#[async_trait]
impl Extension for MetricsExtension {
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        println!("operation:query {:?}", request.query);
        next.run(ctx, request).await
    }
}
