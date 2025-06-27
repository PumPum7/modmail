use actix_web::{get, web, Responder};
use serde::Serialize;
use sqlx::{PgPool, FromRow};

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
    threads_closed: i64,
}

#[derive(Serialize, FromRow)]
struct ResponseTimeMetrics {
    avg_first_response_hours: Option<f64>,
    avg_resolution_time_hours: Option<f64>,
    median_first_response_hours: Option<f64>,
}

#[get("/analytics/overview")]
async fn get_analytics_overview(pool: web::Data<PgPool>) -> impl Responder {
    let total_threads: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM threads")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let open_threads: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM threads WHERE is_open = true")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let closed_threads = total_threads - open_threads;

    let total_messages: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let total_notes: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM notes")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let blocked_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM blocked_users")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    // Calculate average response time (simplified - time between thread creation and first moderator message)
    let avg_response_time_hours: Option<f64> = sqlx::query_scalar(
        r#"
        SELECT AVG(EXTRACT(EPOCH FROM (first_mod_message.created_at - threads.created_at)) / 3600.0)
        FROM threads
        JOIN (
            SELECT tm.thread_id, MIN(m.created_at) as created_at
            FROM thread_messages tm
            JOIN messages m ON tm.message_id = m.id
            WHERE m.author_id != (
                SELECT user_id FROM threads t WHERE t.id = tm.thread_id
            )
            GROUP BY tm.thread_id
        ) first_mod_message ON threads.id = first_mod_message.thread_id
        WHERE threads.created_at IS NOT NULL
        "#
    )
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None);

    let threads_today: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM threads WHERE created_at >= CURRENT_DATE"
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    let threads_this_week: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM threads WHERE created_at >= CURRENT_DATE - INTERVAL '7 days'"
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    let threads_this_month: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM threads WHERE created_at >= CURRENT_DATE - INTERVAL '30 days'"
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    let overview = AnalyticsOverview {
        total_threads,
        open_threads,
        closed_threads,
        total_messages,
        total_notes,
        blocked_users,
        avg_response_time_hours,
        threads_today,
        threads_this_week,
        threads_this_month,
    };

    web::Json(overview)
}

#[get("/analytics/thread-volume")]
async fn get_thread_volume(pool: web::Data<PgPool>) -> impl Responder {
    let volume_data: Vec<ThreadVolumeData> = sqlx::query_as::<_, ThreadVolumeData>(
        r#"
        SELECT 
            DATE(created_at) as "date!: String",
            COUNT(*) as "count!: i64"
        FROM threads 
        WHERE created_at >= CURRENT_DATE - INTERVAL '30 days'
        GROUP BY DATE(created_at)
        ORDER BY DATE(created_at)
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    web::Json(volume_data)
}

#[get("/analytics/moderator-activity")]
async fn get_moderator_activity(pool: web::Data<PgPool>) -> impl Responder {
    let activity_data: Vec<ModeratorActivity> = sqlx::query_as::<_, ModeratorActivity>(
        r#"
        SELECT 
            COALESCE(m.author_tag, n.author_tag, 'Unknown') as "moderator_tag!: String",
            COALESCE(message_count, 0) as "message_count!: i64",
            COALESCE(note_count, 0) as "note_count!: i64",
            0 as "threads_closed!: i64"
        FROM (
            SELECT author_tag, COUNT(*) as message_count
            FROM messages 
            WHERE created_at >= CURRENT_DATE - INTERVAL '30 days'
            GROUP BY author_tag
        ) m
        FULL OUTER JOIN (
            SELECT author_tag, COUNT(*) as note_count
            FROM notes 
            WHERE created_at >= CURRENT_DATE - INTERVAL '30 days'
            GROUP BY author_tag
        ) n ON m.author_tag = n.author_tag
        ORDER BY (COALESCE(message_count, 0) + COALESCE(note_count, 0)) DESC
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    web::Json(activity_data)
}

#[get("/analytics/response-times")]
async fn get_response_times(pool: web::Data<PgPool>) -> impl Responder {
    let avg_first_response_hours: Option<f64> = sqlx::query_scalar(
        r#"
        SELECT AVG(EXTRACT(EPOCH FROM (first_mod_message.created_at - threads.created_at)) / 3600.0)
        FROM threads
        JOIN (
            SELECT tm.thread_id, MIN(m.created_at) as created_at
            FROM thread_messages tm
            JOIN messages m ON tm.message_id = m.id
            WHERE m.author_id != (
                SELECT user_id FROM threads t WHERE t.id = tm.thread_id
            )
            GROUP BY tm.thread_id
        ) first_mod_message ON threads.id = first_mod_message.thread_id
        WHERE threads.created_at >= CURRENT_DATE - INTERVAL '30 days'
        "#
    )
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None);

    let avg_resolution_time_hours: Option<f64> = sqlx::query_scalar(
        r#"
        SELECT AVG(EXTRACT(EPOCH FROM (updated_at - created_at)) / 3600.0)
        FROM threads
        WHERE is_open = false 
        AND created_at >= CURRENT_DATE - INTERVAL '30 days'
        AND updated_at IS NOT NULL
        "#
    )
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None);

    let median_first_response_hours: Option<f64> = sqlx::query_scalar(
        r#"
        SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY EXTRACT(EPOCH FROM (first_mod_message.created_at - threads.created_at)) / 3600.0)
        FROM threads
        JOIN (
            SELECT tm.thread_id, MIN(m.created_at) as created_at
            FROM thread_messages tm
            JOIN messages m ON tm.message_id = m.id
            WHERE m.author_id != (
                SELECT user_id FROM threads t WHERE t.id = tm.thread_id
            )
            GROUP BY tm.thread_id
        ) first_mod_message ON threads.id = first_mod_message.thread_id
        WHERE threads.created_at >= CURRENT_DATE - INTERVAL '30 days'
        "#
    )
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None);

    let metrics = ResponseTimeMetrics {
        avg_first_response_hours,
        avg_resolution_time_hours,
        median_first_response_hours,
    };

    web::Json(metrics)
}
