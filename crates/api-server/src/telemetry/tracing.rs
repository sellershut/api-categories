use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace::Tracer};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt, Layer};

use crate::state::env::extract_variable;

pub fn init_tracer() -> anyhow::Result<Tracer> {
    let tracer = tracer()?;

    let crate_name = env!("CARGO_PKG_NAME");
    let crate_target = crate_name.replace('-', "_");

    let filter = Targets::new()
        .with_target("api_categories", LevelFilter::TRACE)
        .with_default(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{crate_target}=trace,tower_http=debug,axum::rejection=trace,h2=warn,tokio_util=warn,hyper=debug,tonic=debug,tower=debug",
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer.clone()).with_filter(filter))
        .init();

    Ok(tracer)
}

fn tracer() -> anyhow::Result<Tracer> {
    let collector_endpoint =
        extract_variable("OPENTELEMETRY_COLLECTOR_HOST", "http://localhost:4317");

    Ok(opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(collector_endpoint),
        )
        .install_batch(runtime::Tokio)?)
}
