mod auth;
mod error;
mod handlers;
mod models;
mod router;
mod state;

use std::{env, path::PathBuf};

use sqlx::PgPool;
use tokio::fs;
use tracing::info;

use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let admin_token = env::var("ADMIN_TOKEN").expect("ADMIN_TOKEN must be set");
    if admin_token.len() < 32 {
        return Err("ADMIN_TOKEN must contain at least 32 characters".into());
    }

    let upload_dir = PathBuf::from(env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".into()));
    fs::create_dir_all(&upload_dir).await?;
    let state = AppState {
        pool,
        admin_token,
        upload_dir,
    };
    let app = router::build(state);

    let bind = env::var("API_BIND").unwrap_or_else(|_| "127.0.0.1:9000".into());
    let listener = tokio::net::TcpListener::bind(&bind).await?;
    info!(address = %bind, "iie api listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler")
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
}
