use crate::db;
use crate::structs::CreateBlockedUser;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
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

async fn get_blocked_users(State(pool): State<PgPool>, Path(guild_id): Path<String>) -> Response {
    let blocked_users_result = sqlx::query_as::<_, db::BlockedUser>(
        "SELECT * FROM blocked_users WHERE guild_id = $1 ORDER BY created_at DESC",
    )
    .bind(guild_id)
    .fetch_all(&pool)
    .await;

    match blocked_users_result {
        Ok(blocked_users) => (StatusCode::OK, Json(blocked_users)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to fetch blocked users" })),
        )
            .into_response(),
    }
}

async fn block_user(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
    Json(payload): Json<CreateBlockedUser>,
) -> Response {
    let new_blocked_user_result = sqlx::query_as::<_, db::BlockedUser>(
        "INSERT INTO blocked_users (user_id, user_tag, blocked_by, blocked_by_tag, reason, guild_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&payload.user_id)
    .bind(&payload.user_tag)
    .bind(&payload.blocked_by)
    .bind(&payload.blocked_by_tag)
    .bind(&payload.reason)
    .bind(guild_id)
    .fetch_one(&pool)
    .await;

    match new_blocked_user_result {
        Ok(new_blocked_user) => (StatusCode::CREATED, Json(new_blocked_user)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to block user" })),
        )
            .into_response(),
    }
}

async fn unblock_user(
    State(pool): State<PgPool>,
    Path((guild_id, user_id)): Path<(String, String)>,
) -> Response {
    let delete_result =
        sqlx::query("DELETE FROM blocked_users WHERE user_id = $1 AND guild_id = $2")
            .bind(user_id)
            .bind(guild_id)
            .execute(&pool)
            .await;

    match delete_result {
        Ok(result) if result.rows_affected() > 0 => (StatusCode::NO_CONTENT).into_response(),
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "User not found in blocked list" })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to unblock user" })),
        )
            .into_response(),
    }
}

async fn is_user_blocked(
    State(pool): State<PgPool>,
    Path((guild_id, user_id)): Path<(String, String)>,
) -> Response {
    let blocked_user_result = sqlx::query_as::<_, db::BlockedUser>(
        "SELECT * FROM blocked_users WHERE user_id = $1 AND guild_id = $2",
    )
    .bind(user_id)
    .bind(guild_id)
    .fetch_optional(&pool)
    .await;

    match blocked_user_result {
        Ok(Some(user)) => (
            StatusCode::OK,
            Json(serde_json::json!({ "blocked": true, "user": user })),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::OK,
            Json(serde_json::json!({ "blocked": false })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to check if user is blocked" })),
        )
            .into_response(),
    }
}
