use crate::db;
use crate::structs::CreateMessage;
use actix_web::{get, post, web, Responder, Result};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/messages")]
async fn get_messages(pool: web::Data<PgPool>) -> impl Responder {
    let messages = sqlx::query_as::<_, db::Message>("SELECT * FROM messages")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json(messages)
}

#[post("/messages")]
async fn create_message(
    pool: web::Data<PgPool>,
    message: web::Json<CreateMessage>,
) -> Result<impl Responder> {
    let created_at = chrono::Utc::now();
    let id = Uuid::new_v4();
    let attachments = message
        .attachments
        .clone()
        .unwrap_or_else(|| serde_json::json!([]));

    let new_message = sqlx::query_as::<_, db::Message>(
        "INSERT INTO messages (id, author_id, author_tag, content, created_at, attachments) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&id)
    .bind(&message.author_id)
    .bind(&message.author_tag)
    .bind(&message.content)
    .bind(&created_at)
    .bind(&attachments)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    Ok(web::Json(new_message))
}
