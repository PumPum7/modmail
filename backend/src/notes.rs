use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{NewNote, Note};
use crate::schema::{notes, threads};
use crate::structs::CreateNote;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

pub fn note_routes(db_pool: DbPool) -> Router {
    Router::new()
        .route(
            "/guilds/:guild_id/threads/:thread_id/notes",
            get(get_thread_notes).post(add_note_to_thread),
        )
        .with_state(db_pool)
}

async fn get_thread_notes(
    State(pool): State<DbPool>,
    Path((guild_id_path, thread_id_path)): Path<(String, i32)>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;

    let thread_exists = threads::table
        .filter(
            threads::id
                .eq(thread_id_path)
                .and(threads::guild_id.eq(guild_id_path)),
        )
        .select(threads::id)
        .first::<i32>(&mut conn)
        .optional()?
        .is_some();

    if !thread_exists {
        return Err(AppError::Anyhow(anyhow::anyhow!("Thread not found")));
    }

    let results = notes::table
        .filter(notes::thread_id.eq(thread_id_path))
        .order(notes::created_at.asc())
        .select(Note::as_select())
        .load(&mut conn)?;

    Ok((StatusCode::OK, Json(results)))
}

async fn add_note_to_thread(
    State(pool): State<DbPool>,
    Path((guild_id_path, thread_id_path)): Path<(String, i32)>,
    Json(payload): Json<CreateNote>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;

    let thread_exists = threads::table
        .filter(
            threads::id
                .eq(thread_id_path)
                .and(threads::guild_id.eq(&guild_id_path)),
        )
        .select(threads::id)
        .first::<i32>(&mut conn)
        .optional()?
        .is_some();

    if !thread_exists {
        return Err(AppError::Anyhow(anyhow::anyhow!("Thread not found")));
    }

    let new_note = NewNote {
        id: Uuid::new_v4(),
        thread_id: thread_id_path,
        author_id: &payload.author_id,
        author_tag: &payload.author_tag,
        content: &payload.content,
        created_at: Utc::now(),
        guild_id: &guild_id_path,
    };

    let result = diesel::insert_into(notes::table)
        .values(&new_note)
        .returning(Note::as_returning())
        .get_result(&mut conn)?;

    Ok((StatusCode::CREATED, Json(result)))
}
