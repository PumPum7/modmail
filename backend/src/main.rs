use axum::{routing::get, Router};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod analytics;
mod blocked_users;
mod db;
mod guild_configs;
mod macros;
mod messages;
mod notes;
mod servers;
mod structs;
mod threads;

async fn health_check() -> String {
    "OK".to_string()
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::connect(&database_url).await.unwrap();

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
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
