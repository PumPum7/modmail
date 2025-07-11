use crate::db;
use crate::errors::AppError;
use crate::structs::CreateNote;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use sqlx::PgPool;
use uuid::Uuid;

pub fn note_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route(
            "/guilds/:guild_id/threads/:thread_id/notes",
            get(get_thread_notes).post(add_note_to_thread),
        )
        .with_state(db_pool)
}

async fn get_thread_notes(
    State(pool): State<PgPool>,
    Path((guild_id, thread_id)): Path<(String, i32)>,
) -> Result<impl IntoResponse, AppError> {
    let thread_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM threads WHERE id = $1 AND guild_id = $2)")
            .bind(thread_id)
            .bind(guild_id)
            .fetch_one(&pool)
            .await?;

    if !thread_exists {
        return Err(AppError::Anyhow(anyhow::anyhow!("Thread not found")));
    }

    let notes = sqlx::query_as::<_, db::Note>(
        "SELECT * FROM notes WHERE thread_id = $1 ORDER BY created_at ASC",
    )
    .bind(thread_id)
    .fetch_all(&pool)
    .await?;

    Ok((StatusCode::OK, Json(notes)))
}

async fn add_note_to_thread(
    State(pool): State<PgPool>,
    Path((guild_id, thread_id)): Path<(String, i32)>,
    Json(payload): Json<CreateNote>,
) -> Result<impl IntoResponse, AppError> {
    let thread_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM threads WHERE id = $1 AND guild_id = $2)")
            .bind(thread_id)
            .bind(guild_id.clone())
            .fetch_one(&pool)
            .await?;

    if !thread_exists {
        return Err(AppError::Anyhow(anyhow::anyhow!("Thread not found")));
    }

    let note_id = Uuid::new_v4();
    let created_at = chrono::Utc::now();

    let new_note = sqlx::query_as::<_, db::Note>(
        "INSERT INTO notes (id, thread_id, author_id, author_tag, content, created_at, guild_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(note_id)
    .bind(thread_id)
    .bind(&payload.author_id)
    .bind(&payload.author_tag)
    .bind(&payload.content)
    .bind(created_at)
    .bind(guild_id)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(new_note)))
}
