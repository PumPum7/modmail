use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

pub fn analytics_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route(
            "/guilds/:guild_id/analytics/overview",
            get(get_analytics_overview),
        )
        .route(
            "/guilds/:guild_id/analytics/thread-volume",
            get(get_thread_volume),
        )
        .route(
            "/guilds/:guild_id/analytics/moderator-activity",
            get(get_moderator_activity),
        )
        .route(
            "/guilds/:guild_id/analytics/response-times",
            get(get_response_times),
        )
        .route(
            "/guilds/:guild_id/analytics/refresh",
            post(refresh_analytics),
        )
        .with_state(db_pool)
}

#[derive(Serialize, FromRow)]
struct AnalyticsOverview {
    total_threads: i64,
    open_threads: i64,
    closed_threads: i64,
    total_messages: i64,
    total_notes: i64,
    blocked_users: i64,
    avg_response_time_hours: Option<f64>,
    threads_today: i64,
    threads_this_week: i64,
    threads_this_month: i64,
}

#[derive(Serialize, FromRow)]
struct ThreadVolumeData {
    date: String,
    count: i64,
}

#[derive(Serialize, FromRow)]
struct ModeratorActivity {
    moderator_tag: String,
    message_count: i64,
    note_count: i64,
    threads_closed: i32,
}

#[derive(Serialize, FromRow)]
struct ResponseTimeMetrics {
    avg_first_response_hours: Option<f64>,
    avg_resolution_time_hours: Option<f64>,
    median_first_response_hours: Option<f64>,
}

async fn get_analytics_overview(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
) -> Response {
    let total_threads: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM threads WHERE guild_id = $1")
        .bind(guild_id.clone())
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let open_threads: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM threads WHERE is_open = true AND guild_id = $1")
            .bind(guild_id.clone())
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

    let closed_threads = total_threads - open_threads;

    // Calculate time-based thread counts
    let threads_today: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM threads WHERE guild_id = $1 AND created_at >= CURRENT_DATE",
    )
    .bind(guild_id.clone())
    .fetch_one(&pool)
    .await
    .unwrap_or(0);

    let threads_this_week: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM threads WHERE guild_id = $1 AND created_at >= CURRENT_DATE - INTERVAL '7 days'"
    )
    .bind(guild_id.clone())
    .fetch_one(&pool)
    .await
    .unwrap_or(0);

    let threads_this_month: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM threads WHERE guild_id = $1 AND created_at >= CURRENT_DATE - INTERVAL '30 days'"
    )
    .bind(guild_id.clone())
    .fetch_one(&pool)
    .await
    .unwrap_or(0);

    // Calculate average response time (hours between thread creation and first message)
    let avg_response_time_hours: Option<f64> = sqlx::query_scalar(
        r#"
        WITH first_responses AS (
            SELECT 
                t.id,
                t.created_at as thread_created,
                MIN(m.created_at) as first_message
            FROM threads t
            INNER JOIN thread_messages tm ON t.id = tm.thread_id
            INNER JOIN messages m ON tm.message_id = m.id
            WHERE t.guild_id = $1 
                AND t.created_at >= CURRENT_DATE - INTERVAL '30 days'
                AND m.author_id != t.user_id -- Exclude user's own messages
            GROUP BY t.id, t.created_at
        )
        SELECT AVG(EXTRACT(EPOCH FROM (first_message - thread_created)) / 3600.0)
        FROM first_responses
        WHERE first_message > thread_created
        "#,
    )
    .bind(guild_id.clone())
    .fetch_one(&pool)
    .await
    .ok()
    .flatten();

    let (total_messages, total_notes, blocked_users) = tokio::join!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM messages WHERE guild_id = $1")
            .bind(guild_id.clone())
            .fetch_one(&pool),
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM notes WHERE guild_id = $1")
            .bind(guild_id.clone())
            .fetch_one(&pool),
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM blocked_users WHERE guild_id = $1")
            .bind(guild_id.clone())
            .fetch_one(&pool)
    );

    let overview = AnalyticsOverview {
        total_threads,
        open_threads,
        closed_threads,
        total_messages: total_messages.unwrap_or(0),
        total_notes: total_notes.unwrap_or(0),
        blocked_users: blocked_users.unwrap_or(0),
        avg_response_time_hours,
        threads_today,
        threads_this_week,
        threads_this_month,
    };

    (StatusCode::OK, Json(overview)).into_response()
}

async fn get_thread_volume(State(pool): State<PgPool>, Path(guild_id): Path<String>) -> Response {
    let volume_data_result = sqlx::query_as::<_, ThreadVolumeData>(
        r#"
        SELECT 
            DATE(created_at)::TEXT as "date",
            COUNT(*) as "count"
        FROM threads 
        WHERE created_at >= CURRENT_DATE - INTERVAL '30 days' AND guild_id = $1
        GROUP BY DATE(created_at)
        ORDER BY DATE(created_at) DESC
        "#,
    )
    .bind(guild_id)
    .fetch_all(&pool)
    .await;

    match volume_data_result {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to fetch thread volume" })),
        )
            .into_response(),
    }
}

async fn get_moderator_activity(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
) -> Response {
    let activity_data_result = sqlx::query_as::<_, ModeratorActivity>(
        r#"
        SELECT 
            COALESCE(m.author_tag, n.author_tag, 'Unknown') as "moderator_tag",
            COALESCE(message_count, 0) as "message_count",
            COALESCE(note_count, 0) as "note_count",
            0 as "threads_closed"
        FROM (
            SELECT author_tag, COUNT(*) as message_count
            FROM messages 
            WHERE created_at >= CURRENT_DATE - INTERVAL '30 days' AND guild_id = $1
            GROUP BY author_tag
        ) m
        FULL OUTER JOIN (
            SELECT author_tag, COUNT(*) as note_count
            FROM notes 
            WHERE created_at >= CURRENT_DATE - INTERVAL '30 days' AND guild_id = $1
            GROUP BY author_tag
        ) n ON m.author_tag = n.author_tag
        ORDER BY (COALESCE(message_count, 0) + COALESCE(note_count, 0)) DESC
        "#,
    )
    .bind(guild_id)
    .fetch_all(&pool)
    .await;

    match activity_data_result {
        Ok(activity_data) => (StatusCode::OK, Json(activity_data)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to fetch moderator activity" })),
        )
            .into_response(),
    }
}

async fn get_response_times(State(pool): State<PgPool>, Path(guild_id): Path<String>) -> Response {
    // Calculate average first response time (hours between thread creation and first moderator message)
    let avg_first_response_hours: Option<f64> = sqlx::query_scalar(
        r#"
        WITH first_responses AS (
            SELECT 
                t.id,
                t.created_at as thread_created,
                MIN(m.created_at) as first_response
            FROM threads t
            INNER JOIN thread_messages tm ON t.id = tm.thread_id
            INNER JOIN messages m ON tm.message_id = m.id
            WHERE t.guild_id = $1 
                AND t.created_at >= CURRENT_DATE - INTERVAL '30 days'
                AND m.author_id != t.user_id -- Exclude user's own messages
            GROUP BY t.id, t.created_at
        )
        SELECT AVG(EXTRACT(EPOCH FROM (first_response - thread_created)) / 3600.0)
        FROM first_responses
        WHERE first_response > thread_created
        "#,
    )
    .bind(guild_id.clone())
    .fetch_one(&pool)
    .await
    .ok()
    .flatten();

    // Calculate average resolution time (hours between thread creation and closure)
    let avg_resolution_time_hours: Option<f64> = sqlx::query_scalar(
        r#"
        SELECT AVG(EXTRACT(EPOCH FROM (updated_at - created_at)) / 3600.0)
        FROM threads 
        WHERE guild_id = $1 
            AND is_open = false 
            AND created_at >= CURRENT_DATE - INTERVAL '30 days'
            AND updated_at > created_at
        "#,
    )
    .bind(guild_id.clone())
    .fetch_one(&pool)
    .await
    .ok()
    .flatten();

    // Calculate median first response time
    let median_first_response_hours: Option<f64> = sqlx::query_scalar(
        r#"
        WITH first_responses AS (
            SELECT 
                EXTRACT(EPOCH FROM (MIN(m.created_at) - t.created_at)) / 3600.0 as response_hours
            FROM threads t
            INNER JOIN thread_messages tm ON t.id = tm.thread_id
            INNER JOIN messages m ON tm.message_id = m.id
            WHERE t.guild_id = $1 
                AND t.created_at >= CURRENT_DATE - INTERVAL '30 days'
                AND m.author_id != t.user_id -- Exclude user's own messages
                AND m.created_at > t.created_at
            GROUP BY t.id, t.created_at
        )
        SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY response_hours)
        FROM first_responses
        WHERE response_hours > 0
        "#,
    )
    .bind(guild_id.clone())
    .fetch_one(&pool)
    .await
    .ok()
    .flatten();

    let metrics = ResponseTimeMetrics {
        avg_first_response_hours,
        avg_resolution_time_hours,
        median_first_response_hours,
    };

    (StatusCode::OK, Json(metrics)).into_response()
}

async fn refresh_analytics(State(pool): State<PgPool>, Path(guild_id): Path<String>) -> Response {
    // For now, we can use this to validate data integrity and return basic stats
    let validation_result = sqlx::query(
        r#"
        SELECT 
            COUNT(DISTINCT t.id) as total_threads,
            COUNT(DISTINCT tm.message_id) as linked_messages,
            COUNT(DISTINCT n.id) as total_notes
        FROM threads t
        LEFT JOIN thread_messages tm ON t.id = tm.thread_id
        LEFT JOIN notes n ON t.id = n.thread_id
        WHERE t.guild_id = $1
        "#,
    )
    .bind(guild_id.clone())
    .fetch_one(&pool)
    .await;

    match validation_result {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "success": "Analytics refreshed successfully",
                "guild_id": guild_id,
                "timestamp": chrono::Utc::now().to_rfc3339()
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Failed to refresh analytics",
                "details": e.to_string()
            })),
        )
            .into_response(),
    }
}
