use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{Message, NewMessage, NewThread, Thread};
use crate::schema::{messages, thread_messages, threads};
use crate::structs::{CloseThread, CreateMessage, CreateThread, UpdateThreadUrgency};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Router,
};
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
struct PaginationQuery {
    page: Option<i64>,
    limit: Option<i64>,
}

pub fn thread_routes(db_pool: DbPool) -> Router {
    Router::new()
        .route(
            "/guilds/{guild_id}/threads",
            get(get_threads).post(create_thread),
        )
        .route("/guilds/{guild_id}/threads/{thread_id}", get(get_thread))
        .route(
            "/guilds/{guild_id}/threads/{thread_id}/close",
            post(close_thread),
        )
        .route(
            "/guilds/{guild_id}/threads/{thread_id}/messages",
            post(add_message_to_thread),
        )
        .route(
            "/guilds/{guild_id}/threads/{thread_id}/urgency",
            put(update_thread_urgency),
        )
        .with_state(db_pool)
}

async fn get_threads(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let page = pagination.page.unwrap_or(1).max(1);
    let limit = pagination.limit.unwrap_or(20).min(100).max(1);
    let offset = (page - 1) * limit;

    let thread_results = threads::table
        .filter(threads::guild_id.eq(&guild_id_path))
        .order(threads::id.desc())
        .limit(limit)
        .offset(offset)
        .select(Thread::as_select())
        .load(&mut conn)?;

    let total_count: i64 = threads::table
        .filter(threads::guild_id.eq(guild_id_path))
        .count()
        .get_result(&mut conn)?;

    let total_pages = (total_count + limit - 1) / limit;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "threads": thread_results,
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
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
    Json(payload): Json<CreateThread>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let now = Utc::now();
    let new_thread = NewThread {
        user_id: &payload.user_id,
        thread_id: &payload.thread_id,
        guild_id: &guild_id_path,
        urgency: payload.urgency,
        is_open: true,
        created_at: now,
        updated_at: now,
    };

    let result = diesel::insert_into(threads::table)
        .values(&new_thread)
        .returning(Thread::as_returning())
        .get_result(&mut conn)?;

    Ok((StatusCode::OK, Json(result)))
}

async fn get_thread(
    State(pool): State<DbPool>,
    Path((guild_id_path, thread_id_path)): Path<(String, i32)>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let thread_result = threads::table
        .filter(
            threads::id
                .eq(thread_id_path)
                .and(threads::guild_id.eq(guild_id_path)),
        )
        .select(Thread::as_select())
        .first(&mut conn)?;

    let page = pagination.page.unwrap_or(1).max(1);
    let limit = pagination.limit.unwrap_or(50).min(100).max(1);
    let offset = (page - 1) * limit;

    let message_results = thread_messages::table
        .inner_join(messages::table)
        .filter(thread_messages::thread_id.eq(thread_id_path))
        .order(messages::created_at.asc())
        .limit(limit)
        .offset(offset)
        .select(Message::as_select())
        .load(&mut conn)?;

    let total_count: i64 = thread_messages::table
        .filter(thread_messages::thread_id.eq(thread_id_path))
        .count()
        .get_result(&mut conn)?;

    let total_pages = (total_count + limit - 1) / limit;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "thread": thread_result,
            "messages": message_results,
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
    State(pool): State<DbPool>,
    Path((guild_id_path, thread_id_path)): Path<(String, i32)>,
    Json(payload): Json<CloseThread>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let updated_thread = diesel::update(
        threads::table.filter(
            threads::id
                .eq(thread_id_path)
                .and(threads::guild_id.eq(guild_id_path)),
        ),
    )
    .set(threads::is_open.eq(false))
    .returning(Thread::as_returning())
    .get_result(&mut conn)?;

    let discord_webhook_url = std::env::var("DISCORD_WEBHOOK_URL").ok();

    if let Some(webhook_url) = discord_webhook_url {
        let webhook_payload = serde_json::json!({
            "type": "thread_closed",
            "thread": thread_id_path,
            "closed_by_id": payload.closed_by_id,
            "closed_by_tag": payload.closed_by_tag
        });

        tokio::spawn(async move {
            let client = reqwest::Client::new();
            if let Err(e) = client
                .post(&webhook_url)
                .json(&webhook_payload)
                .send()
                .await
            {
                tracing::error!("Failed to send Discord webhook: {}", e);
            }
        });
    }

    Ok((StatusCode::OK, Json(updated_thread)))
}

async fn add_message_to_thread(
    State(pool): State<DbPool>,
    Path((guild_id_path, thread_id_path)): Path<(String, i32)>,
    Json(payload): Json<CreateMessage>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let new_message = NewMessage {
        id: Uuid::new_v4(),
        author_id: &payload.author_id,
        author_tag: &payload.author_tag,
        content: &payload.content,
        created_at: Utc::now(),
        attachments: payload.attachments,
        guild_id: &guild_id_path,
    };

    let inserted_message: Message = diesel::insert_into(messages::table)
        .values(&new_message)
        .returning(Message::as_returning())
        .get_result(&mut conn)?;

    diesel::insert_into(thread_messages::table)
        .values((
            thread_messages::thread_id.eq(thread_id_path),
            thread_messages::message_id.eq(inserted_message.id),
        ))
        .execute(&mut conn)?;

    Ok((StatusCode::OK, Json(inserted_message)))
}

async fn update_thread_urgency(
    State(pool): State<DbPool>,
    Path((guild_id_path, thread_id_path)): Path<(String, i32)>,
    Json(payload): Json<UpdateThreadUrgency>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let updated_thread = diesel::update(
        threads::table.filter(
            threads::id
                .eq(thread_id_path)
                .and(threads::guild_id.eq(guild_id_path)),
        ),
    )
    .set((
        threads::urgency.eq(payload.urgency),
        threads::updated_at.eq(Utc::now()),
    ))
    .returning(Thread::as_returning())
    .get_result(&mut conn)?;

    Ok((StatusCode::OK, Json(updated_thread)))
}
