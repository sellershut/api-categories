mod state;

use anyhow::Result;
use axum::{response::Html, routing::get, Router};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    let state = state::AppState::try_from_env()?;

    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", state.port)).await?;
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
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
