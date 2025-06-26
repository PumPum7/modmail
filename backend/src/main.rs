use actix_web::{get, post, delete, put, web, App, HttpServer, Responder, Result};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use serde::{Deserialize};
use uuid::Uuid;

mod db;

#[derive(Deserialize)]
struct CreateMessage {
    author_id: String,
    author_tag: String,
    content: String,
}

#[derive(Deserialize)]
struct CreateThread {
    user_id: String,
    thread_id: String,
}

#[derive(Deserialize)]
struct CreateMacro {
    name: String,
    content: String,
}

#[get("/messages")]
async fn get_messages(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let messages = sqlx::query_as::<_, db::Message>("SELECT * FROM messages")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json(messages)
}

#[post("/messages")]
async fn create_message(pool: web::Data<sqlx::PgPool>, message: web::Json<CreateMessage>) -> Result<impl Responder> {
    let id = Uuid::new_v4();
    let created_at = chrono::Utc::now();
    
    sqlx::query("INSERT INTO messages (id, author_id, author_tag, content, created_at) VALUES ($1, $2, $3, $4, $5)")
        .bind(&id)
        .bind(&message.author_id)
        .bind(&message.author_tag)
        .bind(&message.content)
        .bind(&created_at)
        .execute(pool.get_ref())
        .await
        .unwrap();

    let new_message = db::Message {
        id,
        author_id: message.author_id.clone(),
        author_tag: message.author_tag.clone(),
        content: message.content.clone(),
        created_at,
    };

    Ok(web::Json(new_message))
}

#[get("/threads")]
async fn get_threads(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let threads = sqlx::query_as::<_, db::Thread>("SELECT * FROM threads")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json(threads)
}

#[post("/threads")]
async fn create_thread(pool: web::Data<sqlx::PgPool>, thread: web::Json<CreateThread>) -> Result<impl Responder> {
    let new_thread = sqlx::query_as::<_, db::Thread>("INSERT INTO threads (user_id, thread_id) VALUES ($1, $2) RETURNING *")
        .bind(&thread.user_id)
        .bind(&thread.thread_id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    Ok(web::Json(new_thread))
}

#[get("/threads/{id}")]
async fn get_thread(pool: web::Data<sqlx::PgPool>, thread_id: web::Path<i32>) -> impl Responder {
    let thread = sqlx::query_as::<_, db::Thread>("SELECT * FROM threads WHERE id = $1")
        .bind(thread_id.into_inner())
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    let messages = sqlx::query_as::<_, db::Message>("SELECT * FROM messages WHERE id IN (SELECT message_id FROM thread_messages WHERE thread_id = $1)")
        .bind(thread.id)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json((thread, messages))
}

#[post("/threads/{id}/close")]
async fn close_thread(pool: web::Data<sqlx::PgPool>, thread_id: web::Path<i32>) -> Result<impl Responder> {
    let updated_thread = sqlx::query_as::<_, db::Thread>("UPDATE threads SET is_open = FALSE WHERE id = $1 RETURNING *")
        .bind(thread_id.into_inner())
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    Ok(web::Json(updated_thread))
}

#[post("/threads/{id}/messages")]
async fn add_message_to_thread(pool: web::Data<sqlx::PgPool>, path: web::Path<i32>, message: web::Json<CreateMessage>) -> Result<impl Responder> {
    let thread_id = path.into_inner();
    let message_id = Uuid::new_v4();
    let created_at = chrono::Utc::now();
    
    // Create the message
    sqlx::query("INSERT INTO messages (id, author_id, author_tag, content, created_at) VALUES ($1, $2, $3, $4, $5)")
        .bind(&message_id)
        .bind(&message.author_id)
        .bind(&message.author_tag)
        .bind(&message.content)
        .bind(&created_at)
        .execute(pool.get_ref())
        .await
        .unwrap();

    // Link message to thread
    sqlx::query("INSERT INTO thread_messages (thread_id, message_id) VALUES ($1, $2)")
        .bind(thread_id)
        .bind(&message_id)
        .execute(pool.get_ref())
        .await
        .unwrap();

    let new_message = db::Message {
        id: message_id,
        author_id: message.author_id.clone(),
        author_tag: message.author_tag.clone(),
        content: message.content.clone(),
        created_at,
    };

    Ok(web::Json(new_message))
}

#[get("/macros")]
async fn get_macros(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let macros = sqlx::query_as::<_, db::Macro>("SELECT * FROM macros")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json(macros)
}

#[post("/macros")]
async fn create_macro(pool: web::Data<sqlx::PgPool>, macro_data: web::Json<CreateMacro>) -> Result<impl Responder> {
    let new_macro = sqlx::query_as::<_, db::Macro>("INSERT INTO macros (name, content) VALUES ($1, $2) RETURNING *")
        .bind(&macro_data.name)
        .bind(&macro_data.content)
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    Ok(web::Json(new_macro))
}

#[get("/macros/{name}")]
async fn get_macro_by_name(pool: web::Data<sqlx::PgPool>, name: web::Path<String>) -> impl Responder {
    let macro_result = sqlx::query_as::<_, db::Macro>("SELECT * FROM macros WHERE name = $1")
        .bind(name.into_inner())
        .fetch_optional(pool.get_ref())
        .await
        .unwrap();

    match macro_result {
        Some(macro_data) => web::Json(serde_json::json!(macro_data)),
        None => web::Json(serde_json::Value::Null),
    }
}

#[delete("/macros/{name}")]
async fn delete_macro(pool: web::Data<sqlx::PgPool>, name: web::Path<String>) -> Result<impl Responder> {
    let rows_affected = sqlx::query("DELETE FROM macros WHERE name = $1")
        .bind(name.into_inner())
        .execute(pool.get_ref())
        .await
        .unwrap()
        .rows_affected();

    if rows_affected > 0 {
        Ok(web::Json(serde_json::json!({"success": true, "message": "Macro deleted"})))
    } else {
        Ok(web::Json(serde_json::json!({"success": false, "message": "Macro not found"})))
    }
}

#[put("/macros/{name}")]
async fn update_macro(pool: web::Data<sqlx::PgPool>, name: web::Path<String>, macro_data: web::Json<CreateMacro>) -> Result<impl Responder> {
    let updated_macro = sqlx::query_as::<_, db::Macro>("UPDATE macros SET content = $1 WHERE name = $2 RETURNING *")
        .bind(&macro_data.content)
        .bind(name.into_inner())
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    Ok(web::Json(updated_macro))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", database_url);
    let pool = db::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
            .app_data(web::Data::new(pool.clone()))
            .service(get_messages)
            .service(create_message)
            .service(get_threads)
            .service(create_thread)
            .service(get_thread)
            .service(close_thread)
            .service(add_message_to_thread)
            .service(get_macros)
            .service(create_macro)
            .service(get_macro_by_name)
            .service(delete_macro)
            .service(update_macro)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}