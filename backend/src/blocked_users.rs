use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{BlockedUser, NewBlockedUser};
use crate::schema::blocked_users::dsl::*;
use crate::structs::CreateBlockedUser;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get},
    Router,
};
use chrono::Utc;
use diesel::prelude::*;

pub fn blocked_user_routes(db_pool: DbPool) -> Router {
    Router::new()
        .route(
            "/guilds/{guild_id}/blocked-users",
            get(get_blocked_users).post(block_user),
        )
        .route(
            "/guilds/{guild_id}/blocked-users/{user_id}",
            delete(unblock_user).get(is_user_blocked),
        )
        .with_state(db_pool)
}

async fn get_blocked_users(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let results = blocked_users
        .filter(guild_id.eq(guild_id_path))
        .order(created_at.desc())
        .select(BlockedUser::as_select())
        .load(&mut conn)?;

    Ok((StatusCode::OK, Json(results)))
}

async fn block_user(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
    Json(payload): Json<CreateBlockedUser>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let new_blocked_user = NewBlockedUser {
        user_id: &payload.user_id,
        user_tag: &payload.user_tag,
        blocked_by: &payload.blocked_by,
        blocked_by_tag: &payload.blocked_by_tag,
        reason: payload.reason,
        created_at: Utc::now(),
        guild_id: &guild_id_path,
    };

    let result = diesel::insert_into(blocked_users)
        .values(&new_blocked_user)
        .returning(BlockedUser::as_returning())
        .get_result(&mut conn)?;

    Ok((StatusCode::CREATED, Json(result)))
}

async fn unblock_user(
    State(pool): State<DbPool>,
    Path((guild_id_path, user_id_path)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let num_deleted = diesel::delete(
        blocked_users.filter(user_id.eq(user_id_path).and(guild_id.eq(guild_id_path))),
    )
    .execute(&mut conn)?;

    if num_deleted > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::Anyhow(anyhow::anyhow!("User not found")))
    }
}

async fn is_user_blocked(
    State(pool): State<DbPool>,
    Path((guild_id_path, user_id_path)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let result = blocked_users
        .filter(user_id.eq(user_id_path).and(guild_id.eq(guild_id_path)))
        .select(BlockedUser::as_select())
        .first(&mut conn)
        .optional()?;

    if let Some(user) = result {
        Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "blocked": true, "user": user })),
        ))
    } else {
        Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "blocked": false })),
        ))
    }
}
