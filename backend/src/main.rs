use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod blocked_users;
mod db;
mod macros;
mod messages;
mod notes;
mod structs;
mod threads;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(messages::get_messages)
            .service(messages::create_message)
            .service(threads::get_threads)
            .service(threads::create_thread)
            .service(threads::get_thread)
            .service(threads::close_thread)
            .service(threads::add_message_to_thread)
            .service(notes::get_thread_notes)
            .service(notes::add_note_to_thread)
            .service(blocked_users::get_blocked_users)
            .service(blocked_users::block_user)
            .service(blocked_users::unblock_user)
            .service(blocked_users::is_user_blocked)
            .service(macros::get_macros)
            .service(macros::create_macro)
            .service(macros::get_macro_by_name)
            .service(macros::delete_macro)
            .service(macros::update_macro)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
