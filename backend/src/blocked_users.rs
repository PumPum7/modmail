use crate::db;
use crate::errors::AppError;
use crate::structs::CreateBlockedUser;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get},
    Router,
};
use sqlx::PgPool;

pub fn blocked_user_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route(
            "/guilds/:guild_id/blocked-users",
            get(get_blocked_users).post(block_user),
        )
        .route(
            "/guilds/:guild_id/blocked-users/:user_id",
            delete(unblock_user).get(is_user_blocked),
        )
        .with_state(db_pool)
}

async fn get_blocked_users(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let blocked_users = sqlx::query_as::<_, db::BlockedUser>(
        "SELECT * FROM blocked_users WHERE guild_id = $1 ORDER BY created_at DESC",
    )
    .bind(guild_id)
    .fetch_all(&pool)
    .await?;

    Ok((StatusCode::OK, Json(blocked_users)))
}

async fn block_user(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
    Json(payload): Json<CreateBlockedUser>,
) -> Result<impl IntoResponse, AppError> {
    let new_blocked_user = sqlx::query_as::<_, db::BlockedUser>(
        "INSERT INTO blocked_users (user_id, user_tag, blocked_by, blocked_by_tag, reason, guild_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&payload.user_id)
    .bind(&payload.user_tag)
    .bind(&payload.blocked_by)
    .bind(&payload.blocked_by_tag)
    .bind(&payload.reason)
    .bind(guild_id)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(new_blocked_user)))
}

async fn unblock_user(
    State(pool): State<PgPool>,
    Path((guild_id, user_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query("DELETE FROM blocked_users WHERE user_id = $1 AND guild_id = $2")
        .bind(user_id)
        .bind(guild_id)
        .execute(&pool)
        .await?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::Anyhow(anyhow::anyhow!("User not found")))
    }
}

async fn is_user_blocked(
    State(pool): State<PgPool>,
    Path((guild_id, user_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let blocked_user = sqlx::query_as::<_, db::BlockedUser>(
        "SELECT * FROM blocked_users WHERE user_id = $1 AND guild_id = $2",
    )
    .bind(user_id)
    .bind(guild_id)
    .fetch_optional(&pool)
    .await?;

    if let Some(user) = blocked_user {
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
