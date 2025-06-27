use crate::db;
use crate::structs::CreateNote;
use actix_web::{get, post, web, Responder, Result};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/threads/{id}/notes")]
async fn get_thread_notes(pool: web::Data<PgPool>, thread_id: web::Path<i32>) -> impl Responder {
    let notes = sqlx::query_as::<_, db::Note>("SELECT * FROM notes WHERE thread_id = $1 ORDER BY created_at ASC")
        .bind(thread_id.into_inner())
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json(notes)
}

#[post("/threads/{id}/notes")]
async fn add_note_to_thread(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    note: web::Json<CreateNote>,
) -> Result<impl Responder> {
    let thread_id = path.into_inner();
    let note_id = Uuid::new_v4();
    let created_at = chrono::Utc::now();

    sqlx::query("INSERT INTO notes (id, thread_id, author_id, author_tag, content, created_at) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&note_id)
        .bind(thread_id)
        .bind(&note.author_id)
        .bind(&note.author_tag)
        .bind(&note.content)
        .bind(&created_at)
        .execute(pool.get_ref())
        .await
        .unwrap();

    let new_note = db::Note {
        id: note_id,
        thread_id,
        author_id: note.author_id.clone(),
        author_tag: note.author_tag.clone(),
        content: note.content.clone(),
        created_at,
    };

    Ok(web::Json(new_note))
}
