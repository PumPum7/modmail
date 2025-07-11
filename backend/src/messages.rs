use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{Message, NewMessage};
use crate::schema::messages::dsl::*;
use crate::structs::CreateMessage;
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

pub fn message_routes(db_pool: DbPool) -> Router {
    Router::new()
        .route(
            "/guilds/:guild_id/messages",
            get(get_messages).post(create_message),
        )
        .with_state(db_pool)
}

async fn get_messages(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let results = messages
        .filter(guild_id.eq(guild_id_path))
        .select(Message::as_select())
        .load(&mut conn)?;

    Ok((StatusCode::OK, Json(results)))
}

async fn create_message(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
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

    let result = diesel::insert_into(messages)
        .values(&new_message)
        .returning(Message::as_returning())
        .get_result(&mut conn)?;

    Ok((StatusCode::CREATED, Json(result)))
}
