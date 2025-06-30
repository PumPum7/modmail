use crate::db;
use crate::structs::CreateBlockedUser;
use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use sqlx::PgPool;

#[get("/blocked-users")]
async fn get_blocked_users(pool: web::Data<PgPool>) -> impl Responder {
    let blocked_users_result = sqlx::query_as::<_, db::BlockedUser>(
        "SELECT * FROM blocked_users ORDER BY created_at DESC",
    )
    .fetch_all(pool.get_ref())
    .await;

    match blocked_users_result {
        Ok(blocked_users) => HttpResponse::Ok().json(blocked_users),
        Err(e) => {
            eprintln!("Database error fetching blocked users: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch blocked users"
            }))
        }
    }
}

#[post("/blocked-users")]
async fn block_user(
    pool: web::Data<PgPool>,
    blocked_user: web::Json<CreateBlockedUser>,
) -> Result<impl Responder> {
    // Validate user ID formats (Discord IDs are numeric)
    if !blocked_user.user_id.chars().all(|c| c.is_ascii_digit()) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid user ID format"
        })));
    }

    if !blocked_user.blocked_by.chars().all(|c| c.is_ascii_digit()) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid blocked_by ID format"
        })));
    }

    let new_blocked_user_result = sqlx::query_as::<_, db::BlockedUser>(
        "INSERT INTO blocked_users (user_id, user_tag, blocked_by, blocked_by_tag, reason) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(&blocked_user.user_id)
    .bind(&blocked_user.user_tag)
    .bind(&blocked_user.blocked_by)
    .bind(&blocked_user.blocked_by_tag)
    .bind(&blocked_user.reason)
    .fetch_one(pool.get_ref())
    .await;

    match new_blocked_user_result {
        Ok(new_blocked_user) => Ok(HttpResponse::Ok().json(new_blocked_user)),
        Err(sqlx::Error::Database(db_err)) => {
            if let Some(constraint) = db_err.constraint() {
                match constraint {
                    "blocked_users_user_id_key" => {
                        Ok(HttpResponse::Conflict().json(serde_json::json!({
                            "error": "User is already blocked"
                        })))
                    }
                    "chk_user_id_format" => {
                        Ok(HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Invalid user ID format"
                        })))
                    }
                    "chk_blocked_by_format" => {
                        Ok(HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Invalid blocked_by ID format"
                        })))
                    }
                    _ => {
                        eprintln!("Database constraint violation: {}", constraint);
                        Ok(HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Data validation failed"
                        })))
                    }
                }
            } else {
                eprintln!("Database error blocking user: {}", db_err);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to block user"
                })))
            }
        }
        Err(e) => {
            eprintln!("Database error blocking user: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to block user"
            })))
        }
    }
}

#[delete("/blocked-users/{user_id}")]
async fn unblock_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<String>,
) -> Result<impl Responder> {
    let delete_result = sqlx::query("DELETE FROM blocked_users WHERE user_id = $1")
        .bind(user_id.into_inner())
        .execute(pool.get_ref())
        .await;

    match delete_result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(HttpResponse::Ok()
                    .json(serde_json::json!({"success": true, "message": "User unblocked"})))
            } else {
                Ok(HttpResponse::NotFound().json(
                    serde_json::json!({"success": false, "message": "User not found in blocked list"}),
                ))
            }
        }
        Err(e) => {
            eprintln!("Database error unblocking user: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to unblock user"
            })))
        }
    }
}

#[get("/blocked-users/{user_id}")]
async fn is_user_blocked(pool: web::Data<PgPool>, user_id: web::Path<String>) -> impl Responder {
    let blocked_user_result =
        sqlx::query_as::<_, db::BlockedUser>("SELECT * FROM blocked_users WHERE user_id = $1")
            .bind(user_id.into_inner())
            .fetch_optional(pool.get_ref())
            .await;

    match blocked_user_result {
        Ok(Some(user)) => {
            HttpResponse::Ok().json(serde_json::json!({"blocked": true, "user": user}))
        }
        Ok(None) => HttpResponse::Ok().json(serde_json::json!({"blocked": false})),
        Err(e) => {
            eprintln!("Database error checking blocked user: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check if user is blocked"
            }))
        }
    }
}
