use opentelemetry_sdk::trace::Tracer;
use sentry::ClientInitGuard;

pub mod metrics;
mod tracing;

pub fn initialise() -> anyhow::Result<(Tracer, ClientInitGuard)> {
    tracing::init_tracer()
}
