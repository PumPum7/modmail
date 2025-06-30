use crate::db;
use crate::structs::{CloseThread, CreateMessage, CreateThread, UpdateThreadUrgency};
use actix_web::{get, post, put, web, HttpResponse, Responder, Result};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct PaginationQuery {
    page: Option<i64>,
    limit: Option<i64>,
}

#[get("/threads")]
async fn get_threads(
    pool: web::Data<PgPool>,
    query: web::Query<PaginationQuery>,
) -> impl Responder {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).min(100).max(1); // Max 100, min 1
    let offset = (page - 1) * limit;

    let threads_result = sqlx::query_as::<_, db::Thread>(
        "SELECT * FROM threads ORDER BY id DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    let threads = match threads_result {
        Ok(threads) => threads,
        Err(e) => {
            eprintln!("Database error fetching threads: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch threads"
            }));
        }
    };

    let total_count_result: Result<i64, sqlx::Error> =
        sqlx::query_scalar("SELECT COUNT(*) FROM threads")
            .fetch_one(pool.get_ref())
            .await;

    let total_count = match total_count_result {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Database error counting threads: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to count threads"
            }));
        }
    };

    let total_pages = (total_count + limit - 1) / limit;

    HttpResponse::Ok().json(serde_json::json!({
        "threads": threads,
        "pagination": {
            "page": page,
            "limit": limit,
            "total_count": total_count,
            "total_pages": total_pages,
            "has_next": page < total_pages,
            "has_prev": page > 1
        }
    }))
}

#[post("/threads")]
async fn create_thread(
    pool: web::Data<PgPool>,
    thread: web::Json<CreateThread>,
) -> Result<impl Responder> {
    // Validate urgency level
    let urgency = thread.urgency.as_deref().unwrap_or("Medium");
    let valid_urgencies = ["Low", "Medium", "High", "Urgent"];

    if !valid_urgencies.contains(&urgency) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid urgency level. Must be one of: Low, Medium, High, Urgent"
        })));
    }

    // Validate user ID format (Discord IDs are numeric)
    if !thread.user_id.chars().all(|c| c.is_ascii_digit()) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid user ID format"
        })));
    }

    let new_thread_result = sqlx::query_as::<_, db::Thread>(
        "INSERT INTO threads (user_id, thread_id, urgency) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&thread.user_id)
    .bind(&thread.thread_id)
    .bind(urgency)
    .fetch_one(pool.get_ref())
    .await;

    match new_thread_result {
        Ok(new_thread) => Ok(HttpResponse::Ok().json(new_thread)),
        Err(sqlx::Error::Database(db_err)) => {
            // Handle specific constraint violations
            if let Some(constraint) = db_err.constraint() {
                match constraint {
                    "uk_user_thread" => {
                        Ok(HttpResponse::Conflict().json(serde_json::json!({
                            "error": "A thread with this user and thread ID already exists"
                        })))
                    }
                    "idx_threads_user_open" => {
                        Ok(HttpResponse::Conflict().json(serde_json::json!({
                            "error": "User already has an open thread. Please close existing thread first."
                        })))
                    }
                    "chk_urgency" => {
                        Ok(HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Invalid urgency level. Must be one of: Low, Medium, High, Urgent"
                        })))
                    }
                    "chk_user_id_format" => {
                        Ok(HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Invalid user ID format"
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
                eprintln!("Database error creating thread: {}", db_err);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to create thread"
                })))
            }
        }
        Err(e) => {
            eprintln!("Database error creating thread: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create thread"
            })))
        }
    }
}

#[get("/threads/{id}")]
async fn get_thread(
    pool: web::Data<PgPool>,
    thread_id: web::Path<i32>,
    query: web::Query<PaginationQuery>,
) -> impl Responder {
    let thread_result = sqlx::query_as::<_, db::Thread>("SELECT * FROM threads WHERE id = $1")
        .bind(thread_id.into_inner())
        .fetch_one(pool.get_ref())
        .await;

    let thread = match thread_result {
        Ok(thread) => thread,
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Thread not found"
            }));
        }
        Err(e) => {
            eprintln!("Database error fetching thread: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch thread"
            }));
        }
    };

    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(50).min(100).max(1); // Max 100, min 1
    let offset = (page - 1) * limit;

    let messages_result = sqlx::query_as::<_, db::Message>(
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
    .fetch_all(pool.get_ref())
    .await;

    let messages = match messages_result {
        Ok(messages) => messages,
        Err(e) => {
            eprintln!("Database error fetching messages: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch messages"
            }));
        }
    };

    let total_count_result: Result<i64, sqlx::Error> =
        sqlx::query_scalar("SELECT COUNT(*) FROM thread_messages WHERE thread_id = $1")
            .bind(thread.id)
            .fetch_one(pool.get_ref())
            .await;

    let total_count = match total_count_result {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Database error counting messages: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to count messages"
            }));
        }
    };

    let total_pages = (total_count + limit - 1) / limit;

    HttpResponse::Ok().json(serde_json::json!({
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
    }))
}

#[post("/threads/{id}/close")]
async fn close_thread(
    pool: web::Data<PgPool>,
    thread_id: web::Path<i32>,
    close_data: Option<web::Json<CloseThread>>,
) -> Result<impl Responder> {
    let thread_id = thread_id.into_inner();

    // Get thread info before closing
    let thread_result = sqlx::query_as::<_, db::Thread>("SELECT * FROM threads WHERE id = $1")
        .bind(thread_id)
        .fetch_one(pool.get_ref())
        .await;

    let thread = match thread_result {
        Ok(thread) => thread,
        Err(sqlx::Error::RowNotFound) => {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "Thread not found"
            })));
        }
        Err(e) => {
            eprintln!("Database error fetching thread: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch thread"
            })));
        }
    };

    let updated_thread_result = sqlx::query_as::<_, db::Thread>(
        "UPDATE threads SET is_open = FALSE WHERE id = $1 RETURNING *",
    )
    .bind(thread_id)
    .fetch_one(pool.get_ref())
    .await;

    let updated_thread = match updated_thread_result {
        Ok(thread) => thread,
        Err(e) => {
            eprintln!("Database error closing thread: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to close thread"
            })));
        }
    };

    if let Some(close_info) = close_data {
        let discord_webhook_url = std::env::var("DISCORD_WEBHOOK_URL").ok();

        if let Some(webhook_url) = discord_webhook_url {
            let payload = serde_json::json!({
                "type": "thread_closed",
                "thread": thread,
                "closed_by_id": close_info.closed_by_id,
                "closed_by_tag": close_info.closed_by_tag
            });

            // Send webhook to Discord bot in background to avoid blocking
            tokio::spawn(async move {
                let client = reqwest::Client::new();
                if let Err(e) = client.post(&webhook_url).json(&payload).send().await {
                    eprintln!("Failed to send Discord webhook: {}", e);
                }
            });
        }
    }

    Ok(HttpResponse::Ok().json(updated_thread))
}

#[post("/threads/{id}/messages")]
async fn add_message_to_thread(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    message: web::Json<CreateMessage>,
) -> Result<impl Responder> {
    // Validate author ID format (Discord IDs are numeric)
    if !message.author_id.chars().all(|c| c.is_ascii_digit()) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid author ID format"
        })));
    }

    let thread_message_id = Uuid::new_v4();
    let created_at = chrono::Utc::now();
    let attachments = message
        .attachments
        .clone()
        .unwrap_or_else(|| serde_json::json!([]));

    let new_message_result = sqlx::query_as::<_, db::Message>(
        "INSERT INTO messages (id, author_id, author_tag, content, created_at, attachments) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&thread_message_id)
    .bind(&message.author_id)
    .bind(&message.author_tag)
    .bind(&message.content)
    .bind(&created_at)
    .bind(&attachments)
    .fetch_one(pool.get_ref())
    .await;

    let new_message = match new_message_result {
        Ok(message) => message,
        Err(sqlx::Error::Database(db_err)) => {
            if let Some(constraint) = db_err.constraint() {
                match constraint {
                    "chk_author_id_format" => {
                        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Invalid author ID format"
                        })));
                    }
                    _ => {
                        eprintln!("Database constraint violation: {}", constraint);
                        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Data validation failed"
                        })));
                    }
                }
            } else {
                eprintln!("Database error creating message: {}", db_err);
                return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to create message"
                })));
            }
        }
        Err(e) => {
            eprintln!("Database error creating message: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create message"
            })));
        }
    };

    let link_result =
        sqlx::query("INSERT INTO thread_messages (thread_id, message_id) VALUES ($1, $2)")
            .bind(path.into_inner())
            .bind(&thread_message_id)
            .execute(pool.get_ref())
            .await;

    if let Err(e) = link_result {
        eprintln!("Database error linking message to thread: {}", e);
        return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to link message to thread"
        })));
    }

    Ok(HttpResponse::Ok().json(new_message))
}

#[put("/threads/{id}/urgency")]
async fn update_thread_urgency(
    pool: web::Data<PgPool>,
    thread_id: web::Path<i32>,
    urgency_data: web::Json<UpdateThreadUrgency>,
) -> Result<impl Responder> {
    let thread_id = thread_id.into_inner();

    // Validate urgency level
    let valid_urgencies = ["Low", "Medium", "High", "Urgent"];
    if !valid_urgencies.contains(&urgency_data.urgency.as_str()) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid urgency level. Must be one of: Low, Medium, High, Urgent"
        })));
    }

    let updated_thread_result = sqlx::query_as::<_, db::Thread>(
        "UPDATE threads SET urgency = $1, updated_at = NOW() WHERE id = $2 RETURNING *",
    )
    .bind(&urgency_data.urgency)
    .bind(thread_id)
    .fetch_one(pool.get_ref())
    .await;

    match updated_thread_result {
        Ok(updated_thread) => Ok(HttpResponse::Ok().json(updated_thread)),
        Err(sqlx::Error::RowNotFound) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Thread not found"
        }))),
        Err(sqlx::Error::Database(db_err)) => {
            if let Some(constraint) = db_err.constraint() {
                match constraint {
                    "chk_urgency" => Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Invalid urgency level. Must be one of: Low, Medium, High, Urgent"
                    }))),
                    _ => {
                        eprintln!("Database constraint violation: {}", constraint);
                        Ok(HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Data validation failed"
                        })))
                    }
                }
            } else {
                eprintln!("Database error updating thread urgency: {}", db_err);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to update thread urgency"
                })))
            }
        }
        Err(e) => {
            eprintln!("Database error updating thread urgency: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update thread urgency"
            })))
        }
    }
}
