use crate::db;
use crate::structs::CreateNote;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/threads/{id}/notes")]
async fn get_thread_notes(pool: web::Data<PgPool>, thread_id: web::Path<i32>) -> impl Responder {
    let notes_result = sqlx::query_as::<_, db::Note>(
        "SELECT * FROM notes WHERE thread_id = $1 ORDER BY created_at ASC",
    )
    .bind(thread_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match notes_result {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(e) => {
            eprintln!("Database error fetching notes: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch notes"
            }))
        }
    }
}

#[post("/threads/{id}/notes")]
async fn add_note_to_thread(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    note: web::Json<CreateNote>,
) -> Result<impl Responder> {
    // Validate author ID format (Discord IDs are numeric)
    if !note.author_id.chars().all(|c| c.is_ascii_digit()) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid author ID format"
        })));
    }

    let thread_id = path.into_inner();
    let note_id = Uuid::new_v4();
    let created_at = chrono::Utc::now();

    let insert_result = sqlx::query("INSERT INTO notes (id, thread_id, author_id, author_tag, content, created_at) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&note_id)
        .bind(thread_id)
        .bind(&note.author_id)
        .bind(&note.author_tag)
        .bind(&note.content)
        .bind(&created_at)
        .execute(pool.get_ref())
        .await;

    match insert_result {
        Ok(_) => {
            let new_note = db::Note {
                id: note_id,
                thread_id,
                author_id: note.author_id.clone(),
                author_tag: note.author_tag.clone(),
                content: note.content.clone(),
                created_at,
            };
            Ok(HttpResponse::Ok().json(new_note))
        }
        Err(sqlx::Error::Database(db_err)) => {
            if let Some(constraint) = db_err.constraint() {
                match constraint {
                    "chk_author_id_format" => {
                        Ok(HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Invalid author ID format"
                        })))
                    }
                    "fk_notes_thread" => Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Thread not found"
                    }))),
                    _ => {
                        eprintln!("Database constraint violation: {}", constraint);
                        Ok(HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Data validation failed"
                        })))
                    }
                }
            } else {
                eprintln!("Database error creating note: {}", db_err);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to create note"
                })))
            }
        }
        Err(e) => {
            eprintln!("Database error creating note: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create note"
            })))
        }
    }
}
