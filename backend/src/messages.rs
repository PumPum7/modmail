use crate::db;
use crate::structs::CreateMessage;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/messages")]
async fn get_messages(pool: web::Data<PgPool>) -> impl Responder {
    let messages_result = sqlx::query_as::<_, db::Message>("SELECT * FROM messages")
        .fetch_all(pool.get_ref())
        .await;

    match messages_result {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(e) => {
            eprintln!("Database error fetching messages: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch messages"
            }))
        }
    }
}

#[post("/messages")]
async fn create_message(
    pool: web::Data<PgPool>,
    message: web::Json<CreateMessage>,
) -> Result<impl Responder> {
    // Validate author ID format (Discord IDs are numeric)
    if !message.author_id.chars().all(|c| c.is_ascii_digit()) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid author ID format"
        })));
    }

    let created_at = chrono::Utc::now();
    let id = Uuid::new_v4();
    let attachments = message
        .attachments
        .clone()
        .unwrap_or_else(|| serde_json::json!([]));

    let new_message_result = sqlx::query_as::<_, db::Message>(
        "INSERT INTO messages (id, author_id, author_tag, content, created_at, attachments) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&id)
    .bind(&message.author_id)
    .bind(&message.author_tag)
    .bind(&message.content)
    .bind(&created_at)
    .bind(&attachments)
    .fetch_one(pool.get_ref())
    .await;

    match new_message_result {
        Ok(new_message) => Ok(HttpResponse::Ok().json(new_message)),
        Err(sqlx::Error::Database(db_err)) => {
            if let Some(constraint) = db_err.constraint() {
                match constraint {
                    "chk_author_id_format" => {
                        Ok(HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Invalid author ID format"
                        })))
                    }
                    _ => {
                        eprintln!("Database constraint violation: {}", constraint);
                        Ok(HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Data validation failed"
                        })))
                    }
                }
            } else {
                eprintln!("Database error creating message: {}", db_err);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to create message"
                })))
            }
        }
        Err(e) => {
            eprintln!("Database error creating message: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create message"
            })))
        }
    }
}
