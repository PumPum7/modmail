use crate::db;
use crate::errors::AppError;
use crate::structs::CreateMessage;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
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

async fn get_messages(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let messages = sqlx::query_as::<_, db::Message>("SELECT * FROM messages WHERE guild_id = $1")
        .bind(guild_id)
        .fetch_all(&pool)
        .await?;

    Ok((StatusCode::OK, Json(messages)))
}

async fn create_message(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
    Json(payload): Json<CreateMessage>,
) -> Result<impl IntoResponse, AppError> {
    let created_at = chrono::Utc::now();
    let id = Uuid::new_v4();
    let attachments = payload
        .attachments
        .clone()
        .unwrap_or_else(|| serde_json::json!([]));

    let new_message = sqlx::query_as::<_, db::Message>(
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
    .await?;

    Ok((StatusCode::CREATED, Json(new_message)))
}
