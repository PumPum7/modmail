use axum::{routing::get, Router};
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod analytics;
mod blocked_users;
mod db;
mod errors;
mod guild_configs;
mod macros;
mod messages;
mod models;
mod notes;
mod schema;
mod servers;
mod structs;
mod threads;

use crate::errors::AppError;

async fn health_check() -> Result<String, AppError> {
    Ok("OK".to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db::establish_connection();

    let app = Router::new()
        .route("/health", get(health_check))
        .merge(messages::message_routes(pool.clone()))
        .merge(threads::thread_routes(pool.clone()))
        .merge(notes::note_routes(pool.clone()))
        .merge(blocked_users::blocked_user_routes(pool.clone()))
        .merge(macros::macro_routes(pool.clone()))
        .merge(analytics::analytics_routes(pool.clone()))
        .merge(servers::server_routes(pool.clone()))
        .merge(guild_configs::guild_config_routes(pool.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
