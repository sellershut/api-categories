mod routes;
mod state;
mod telemetry;

use std::future::ready;

use anyhow::Result;
use async_graphql::extensions::{OpenTelemetry, Tracing};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    http::{header, HeaderValue, Method},
    middleware,
    routing::get,
    Router,
};
use tokio::signal;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::routes::{
    handler,
    middleware::{graphql::Metrics, track_metrics},
};

const SUBSCRIPTION_ENDPOINT: &str = "/ws";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let (tracer, _sentry_guard) = telemetry::initialise()?;

    let state = state::AppState::try_from_env()?;

    let schema_builder = api_interface::ApiSchemaBuilder::new(state.database_credentials())
        .await
        .with_extension(Tracing)
        .with_extension(Metrics)
        .with_extension(OpenTelemetry::new(tracer));

    let schema = schema_builder.build();

    let app = Router::new()
        .route("/", get(handler).post_service(GraphQL::new(schema.clone())))
        .route(
            "/metrics",
            get(move || ready(state.metrics_handle.render())),
        )
        .route_service(SUBSCRIPTION_ENDPOINT, GraphQLSubscription::new(schema))
        .route_layer(middleware::from_fn(track_metrics))
        // If you want to customize the behavior using closures here is how.
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(state.frontend_url.parse::<HeaderValue>()?)
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .allow_methods([Method::GET, Method::POST]),
        );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", state.port)).await?;
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
