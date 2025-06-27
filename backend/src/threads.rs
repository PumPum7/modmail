use crate::db;
use crate::structs::CreateMessage;
use crate::structs::CreateThread;
use actix_web::{get, post, web, Responder, Result};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/threads")]
async fn get_threads(pool: web::Data<PgPool>) -> impl Responder {
    let threads = sqlx::query_as::<_, db::Thread>("SELECT * FROM threads")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json(threads)
}

#[post("/threads")]
async fn create_thread(
    pool: web::Data<PgPool>,
    thread: web::Json<CreateThread>,
) -> Result<impl Responder> {
    let new_thread = sqlx::query_as::<_, db::Thread>(
        "INSERT INTO threads (user_id, thread_id) VALUES ($1, $2) RETURNING *",
    )
    .bind(&thread.user_id)
    .bind(&thread.thread_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    Ok(web::Json(new_thread))
}

#[get("/threads/{id}")]
async fn get_thread(pool: web::Data<PgPool>, thread_id: web::Path<i32>) -> impl Responder {
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
async fn close_thread(
    pool: web::Data<PgPool>,
    thread_id: web::Path<i32>,
) -> Result<impl Responder> {
    let updated_thread = sqlx::query_as::<_, db::Thread>(
        "UPDATE threads SET is_open = FALSE WHERE id = $1 RETURNING *",
    )
    .bind(thread_id.into_inner())
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    Ok(web::Json(updated_thread))
}

#[post("/threads/{id}/messages")]
async fn add_message_to_thread(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    message: web::Json<CreateMessage>,
) -> Result<impl Responder> {
    let thread_message_id = Uuid::new_v4();
    let created_at = chrono::Utc::now();
    let attachments = message
        .attachments
        .clone()
        .unwrap_or_else(|| serde_json::json!([]));

    let new_message = sqlx::query_as::<_, db::Message>(
        "INSERT INTO messages (id, author_id, author_tag, content, created_at, attachments) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&thread_message_id)
    .bind(&message.author_id)
    .bind(&message.author_tag)
    .bind(&message.content)
    .bind(&created_at)
    .bind(&attachments)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    sqlx::query("INSERT INTO thread_messages (thread_id, message_id) VALUES ($1, $2)")
        .bind(path.into_inner())
        .bind(&thread_message_id)
        .execute(pool.get_ref())
        .await
        .unwrap();

    Ok(web::Json(new_message))
}
