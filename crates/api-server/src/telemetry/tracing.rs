use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    runtime,
    trace::{self, Tracer},
    Resource,
};
use opentelemetry_semantic_conventions::resource::{
    DEPLOYMENT_ENVIRONMENT, SERVICE_NAME, SERVICE_VERSION,
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt, Layer};

use crate::state::env::extract_variable;

pub fn init_tracer() -> anyhow::Result<Tracer> {
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_ver = env!("CARGO_PKG_VERSION");

    let tracer = tracer(pkg_name, pkg_ver)?;

    let crate_target = pkg_name.replace('-', "_");

    let filter = Targets::new()
        .with_target("api_categories", LevelFilter::TRACE)
        .with_default(LevelFilter::TRACE);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{crate_target}=trace,api_interface=trace,tower_http=debug,axum::rejection=trace,h2=warn,tokio_util=warn,hyper=debug,tonic=debug,tower=debug",
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer.clone()).with_filter(filter))
        .init();

    Ok(tracer)
}

fn tracer(pkg_name: &str, pkg_ver: &str) -> anyhow::Result<Tracer> {
    let collector_endpoint =
        extract_variable("OPENTELEMETRY_COLLECTOR_HOST", "http://localhost:4317");

    let deployment_env = extract_variable("ENV", "develop");

    Ok(opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(collector_endpoint),
        )
        .with_trace_config(trace::config().with_resource(Resource::new([
            KeyValue::new(SERVICE_NAME, pkg_name.to_owned()),
            KeyValue::new(SERVICE_VERSION, pkg_ver.to_owned()),
            KeyValue::new(DEPLOYMENT_ENVIRONMENT, deployment_env),
        ])))
        .install_batch(runtime::Tokio)?)
}
