mod routes;
mod state;
mod telemetry;

use anyhow::Result;
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    http::{header, HeaderValue, Method},
    routing::get,
    Router,
};
use tokio::signal;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::routes::handler;

const SUBSCRIPTION_ENDPOINT: &str = "/ws";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let (tracer, _sentry_guard) = telemetry::initialise()?;

    let state = state::AppState::try_from_env()?;

    let schema = api_interface::create_schema(tracer, state.database_credentials()).await;

    let app = Router::new()
        .route("/", get(handler).post_service(GraphQL::new(schema.clone())))
        .route_service(SUBSCRIPTION_ENDPOINT, GraphQLSubscription::new(schema))
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
