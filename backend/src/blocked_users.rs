use crate::db;
use crate::structs::CreateBlockedUser;
use actix_web::{delete, get, post, web, Responder, Result};
use sqlx::PgPool;

#[get("/blocked-users")]
async fn get_blocked_users(pool: web::Data<PgPool>) -> impl Responder {
    let blocked_users = sqlx::query_as::<_, db::BlockedUser>(
        "SELECT * FROM blocked_users ORDER BY created_at DESC",
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    web::Json(blocked_users)
}

#[post("/blocked-users")]
async fn block_user(
    pool: web::Data<PgPool>,
    blocked_user: web::Json<CreateBlockedUser>,
) -> Result<impl Responder> {
    let new_blocked_user = sqlx::query_as::<_, db::BlockedUser>(
        "INSERT INTO blocked_users (user_id, user_tag, blocked_by, blocked_by_tag, reason) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(&blocked_user.user_id)
    .bind(&blocked_user.user_tag)
    .bind(&blocked_user.blocked_by)
    .bind(&blocked_user.blocked_by_tag)
    .bind(&blocked_user.reason)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    Ok(web::Json(new_blocked_user))
}

#[delete("/blocked-users/{user_id}")]
async fn unblock_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<String>,
) -> Result<impl Responder> {
    let rows_affected = sqlx::query("DELETE FROM blocked_users WHERE user_id = $1")
        .bind(user_id.into_inner())
        .execute(pool.get_ref())
        .await
        .unwrap()
        .rows_affected();

    if rows_affected > 0 {
        Ok(web::Json(
            serde_json::json!({"success": true, "message": "User unblocked"}),
        ))
    } else {
        Ok(web::Json(
            serde_json::json!({"success": false, "message": "User not found in blocked list"}),
        ))
    }
}

#[get("/blocked-users/{user_id}")]
async fn is_user_blocked(pool: web::Data<PgPool>, user_id: web::Path<String>) -> impl Responder {
    let blocked_user =
        sqlx::query_as::<_, db::BlockedUser>("SELECT * FROM blocked_users WHERE user_id = $1")
            .bind(user_id.into_inner())
            .fetch_optional(pool.get_ref())
            .await
            .unwrap();

    match blocked_user {
        Some(user) => web::Json(serde_json::json!({"blocked": true, "user": user})),
        None => web::Json(serde_json::json!({"blocked": false})),
    }
}
