use crate::db;
use crate::errors::AppError;
use crate::structs::{CloseThread, CreateMessage, CreateThread, UpdateThreadUrgency};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct PaginationQuery {
    page: Option<i64>,
    limit: Option<i64>,
}

pub fn thread_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route(
            "/guilds/:guild_id/threads",
            get(get_threads).post(create_thread),
        )
        .route("/guilds/:guild_id/threads/:thread_id", get(get_thread))
        .route(
            "/guilds/:guild_id/threads/:thread_id/close",
            post(close_thread),
        )
        .route(
            "/guilds/:guild_id/threads/:thread_id/messages",
            post(add_message_to_thread),
        )
        .route(
            "/guilds/:guild_id/threads/:thread_id/urgency",
            put(update_thread_urgency),
        )
        .with_state(db_pool)
}

async fn get_threads(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let page = pagination.page.unwrap_or(1).max(1);
    let limit = pagination.limit.unwrap_or(20).min(100).max(1);
    let offset = (page - 1) * limit;

    let threads = sqlx::query_as::<_, db::Thread>(
        "SELECT * FROM threads WHERE guild_id = $1 ORDER BY id DESC LIMIT $2 OFFSET $3",
    )
    .bind(guild_id.clone())
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await?;

    let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM threads WHERE guild_id = $1")
        .bind(guild_id)
        .fetch_one(&pool)
        .await?;

    let total_pages = (total_count + limit - 1) / limit;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "threads": threads,
            "pagination": {
                "page": page,
                "limit": limit,
                "total_count": total_count,
                "total_pages": total_pages,
                "has_next": page < total_pages,
                "has_prev": page > 1
            }
        })),
    ))
}

async fn create_thread(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
    Json(payload): Json<CreateThread>,
) -> Result<impl IntoResponse, AppError> {
    let urgency = payload.urgency.as_deref().unwrap_or("Medium");

    let new_thread = sqlx::query_as::<_, db::Thread>(
        "INSERT INTO threads (user_id, thread_id, urgency, guild_id) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(&payload.user_id)
    .bind(&payload.thread_id)
    .bind(urgency)
    .bind(guild_id)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::OK, Json(new_thread)))
}

async fn get_thread(
    State(pool): State<PgPool>,
    Path((guild_id, thread_id)): Path<(String, i32)>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let thread =
        sqlx::query_as::<_, db::Thread>("SELECT * FROM threads WHERE id = $1 AND guild_id = $2")
            .bind(thread_id)
            .bind(guild_id)
            .fetch_one(&pool)
            .await?;

    let page = pagination.page.unwrap_or(1).max(1);
    let limit = pagination.limit.unwrap_or(50).min(100).max(1);
    let offset = (page - 1) * limit;

    let messages = sqlx::query_as::<_, db::Message>(
        r#"
        SELECT m.* 
        FROM messages m
        INNER JOIN thread_messages tm ON m.id = tm.message_id
        WHERE tm.thread_id = $1 
        ORDER BY m.created_at ASC 
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(thread.id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await?;

    let total_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM thread_messages WHERE thread_id = $1")
            .bind(thread.id)
            .fetch_one(&pool)
            .await?;

    let total_pages = (total_count + limit - 1) / limit;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "thread": thread,
            "messages": messages,
            "pagination": {
                "page": page,
                "limit": limit,
                "total_count": total_count,
                "total_pages": total_pages,
                "has_next": page < total_pages,
                "has_prev": page > 1
            }
        })),
    ))
}

async fn close_thread(
    State(pool): State<PgPool>,
    Path((guild_id, thread_id)): Path<(String, i32)>,
    Json(payload): Json<CloseThread>,
) -> Result<impl IntoResponse, AppError> {
    let updated_thread = sqlx::query_as::<_, db::Thread>(
        "UPDATE threads SET is_open = FALSE WHERE id = $1 AND guild_id = $2 RETURNING *",
    )
    .bind(thread_id)
    .bind(guild_id)
    .fetch_one(&pool)
    .await?;

    let discord_webhook_url = std::env::var("DISCORD_WEBHOOK_URL").ok();

    if let Some(webhook_url) = discord_webhook_url {
        let payload = serde_json::json!({
            "type": "thread_closed",
            "thread": thread_id,
            "closed_by_id": payload.closed_by_id,
            "closed_by_tag": payload.closed_by_tag
        });

        // Send webhook to Discord bot in background to avoid blocking
        tokio::spawn(async move {
            let client = reqwest::Client::new();
            if let Err(e) = client.post(&webhook_url).json(&payload).send().await {
                tracing::error!("Failed to send Discord webhook: {}", e);
            }
        });
    }

    Ok((StatusCode::OK, Json(updated_thread)))
}

async fn add_message_to_thread(
    State(pool): State<PgPool>,
    Path((guild_id, thread_id)): Path<(String, i32)>,
    Json(payload): Json<CreateMessage>,
) -> Result<impl IntoResponse, AppError> {
    let thread_message_id = Uuid::new_v4();
    let created_at = chrono::Utc::now();
    let attachments = payload.attachments.unwrap_or_else(|| serde_json::json!([]));

    let new_message = sqlx::query_as::<_, db::Message>(
        "INSERT INTO messages (id, author_id, author_tag, content, created_at, attachments, guild_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(&thread_message_id)
    .bind(&payload.author_id)
    .bind(&payload.author_tag)
    .bind(&payload.content)
    .bind(&created_at)
    .bind(&attachments)
    .bind(guild_id)
    .fetch_one(&pool)
    .await?;

    sqlx::query("INSERT INTO thread_messages (thread_id, message_id) VALUES ($1, $2)")
        .bind(thread_id)
        .bind(&thread_message_id)
        .execute(&pool)
        .await?;

    Ok((StatusCode::OK, Json(new_message)))
}

async fn update_thread_urgency(
    State(pool): State<PgPool>,
    Path((guild_id, thread_id)): Path<(String, i32)>,
    Json(payload): Json<UpdateThreadUrgency>,
) -> Result<impl IntoResponse, AppError> {
    let updated_thread = sqlx::query_as::<_, db::Thread>(
        "UPDATE threads SET urgency = $1, updated_at = NOW() WHERE id = $2 AND guild_id = $3 RETURNING *",
    )
    .bind(&payload.urgency)
    .bind(thread_id)
    .bind(guild_id)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::OK, Json(updated_thread)))
}
