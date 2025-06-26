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
