use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod analytics;
mod blocked_users;
mod db;
mod macros;
mod messages;
mod notes;
mod structs;
mod threads;

use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::connect(&database_url).await.unwrap();

    // Clone pool for background task before moving into HttpServer
    let analytics_pool = pool.clone();

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(health_check)
            .service(messages::get_messages)
            .service(messages::create_message)
            .service(threads::get_threads)
            .service(threads::create_thread)
            .service(threads::get_thread)
            .service(threads::close_thread)
            .service(threads::add_message_to_thread)
            .service(threads::update_thread_urgency)
            .service(notes::get_thread_notes)
            .service(notes::add_note_to_thread)
            .service(blocked_users::get_blocked_users)
            .service(blocked_users::block_user)
            .service(blocked_users::unblock_user)
            .service(blocked_users::is_user_blocked)
            .service(macros::get_macros)
            .service(macros::get_quick_access_macros)
            .service(macros::create_macro)
            .service(macros::get_macro_by_name)
            .service(macros::delete_macro)
            .service(macros::update_macro)
            .service(analytics::get_analytics_overview)
            .service(analytics::get_thread_volume)
            .service(analytics::get_moderator_activity)
            .service(analytics::get_response_times)
            .service(analytics::refresh_analytics) // Add new refresh endpoint
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    // Start background task for analytics refresh
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // Refresh every hour
        loop {
            interval.tick().await;
            analytics::auto_refresh_analytics(&analytics_pool).await;
        }
    });

    server.await
}
