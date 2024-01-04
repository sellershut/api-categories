use opentelemetry_sdk::trace::Tracer;

mod tracing;

pub fn initialise() -> anyhow::Result<Tracer> {
    tracing::init_tracer()
}
