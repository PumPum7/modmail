use crate::db;
use crate::structs::CreateMessage;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use sqlx::PgPool;
use uuid::Uuid;

pub fn message_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route(
            "/guilds/:guild_id/messages",
            get(get_messages).post(create_message),
        )
        .with_state(db_pool)
}

async fn get_messages(State(pool): State<PgPool>, Path(guild_id): Path<String>) -> Response {
    let messages_result =
        sqlx::query_as::<_, db::Message>("SELECT * FROM messages WHERE guild_id = $1")
            .bind(guild_id)
            .fetch_all(&pool)
            .await;

    match messages_result {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to fetch messages" })),
        )
            .into_response(),
    }
}

async fn create_message(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
    Json(payload): Json<CreateMessage>,
) -> Response {
    let created_at = chrono::Utc::now();
    let id = Uuid::new_v4();
    let attachments = payload
        .attachments
        .clone()
        .unwrap_or_else(|| serde_json::json!([]));

    let new_message_result = sqlx::query_as::<_, db::Message>(
        "INSERT INTO messages (id, author_id, author_tag, content, created_at, attachments, guild_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(&id)
    .bind(&payload.author_id)
    .bind(&payload.author_tag)
    .bind(&payload.content)
    .bind(&created_at)
    .bind(&attachments)
    .bind(guild_id)
    .fetch_one(&pool)
    .await;

    match new_message_result {
        Ok(new_message) => (StatusCode::CREATED, Json(new_message)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to create message" })),
        )
            .into_response(),
    }
}
