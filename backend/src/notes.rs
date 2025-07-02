use crate::db;
use crate::structs::CreateNote;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
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
) -> Response {
    let thread_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM threads WHERE id = $1 AND guild_id = $2)",
    )
    .bind(thread_id)
    .bind(guild_id)
    .fetch_one(&pool)
    .await
    .unwrap_or(false);

    if !thread_exists {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Thread not found in this guild" })),
        )
            .into_response();
    }

    let notes_result = sqlx::query_as::<_, db::Note>(
        "SELECT * FROM notes WHERE thread_id = $1 ORDER BY created_at ASC",
    )
    .bind(thread_id)
    .fetch_all(&pool)
    .await;

    match notes_result {
        Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to fetch notes" })),
        )
            .into_response(),
    }
}

async fn add_note_to_thread(
    State(pool): State<PgPool>,
    Path((guild_id, thread_id)): Path<(String, i32)>,
    Json(payload): Json<CreateNote>,
) -> Response {
    let thread_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM threads WHERE id = $1 AND guild_id = $2)",
    )
    .bind(thread_id)
    .bind(guild_id.clone())
    .fetch_one(&pool)
    .await
    .unwrap_or(false);

    if !thread_exists {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Thread not found in this guild" })),
        )
            .into_response();
    }

    let note_id = Uuid::new_v4();
    let created_at = chrono::Utc::now();

    let new_note_result = sqlx::query_as::<_, db::Note>(
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
    .await;

    match new_note_result {
        Ok(new_note) => (StatusCode::CREATED, Json(new_note)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to create note" })),
        )
            .into_response(),
    }
}
