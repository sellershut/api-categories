mod state;
mod telemetry;

use anyhow::Result;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let tracer = telemetry::initialise()?;

    let state = state::AppState::try_from_env()?;

    let schema = api_interface::create_schema(tracer, state.database_credentials()).await;

    let app = Router::new()
        .route("/", get(handler).post_service(GraphQL::new(schema.clone())))
        .route_service("/ws", GraphQLSubscription::new(schema));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", state.port)).await?;
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn handler() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
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
