use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
struct AnalyticsSummary {
    total_threads: Option<i64>,
    open_threads: Option<i64>,
    closed_threads: Option<i64>,
}

#[derive(Serialize, FromRow)]
struct AnalyticsSummaryTimeCounts {
    threads_today: Option<i64>,
    threads_this_week: Option<i64>,
    threads_this_month: Option<i64>,
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

#[get("/analytics/overview")]
async fn get_analytics_overview(pool: web::Data<PgPool>) -> impl Responder {
    // Try to get basic counts from materialized view first for better performance
    let view_data_result = sqlx::query_as::<_, AnalyticsSummary>(
        r#"
        SELECT 
            SUM(thread_count) as total_threads,
            SUM(open_threads) as open_threads,
            SUM(closed_threads) as closed_threads
        FROM analytics_summary
        "#,
    )
    .fetch_optional(pool.get_ref())
    .await;

    let (total_threads, open_threads, closed_threads) = match view_data_result {
        Ok(Some(data)) if data.total_threads.is_some() => (
            data.total_threads.unwrap_or(0),
            data.open_threads.unwrap_or(0),
            data.closed_threads.unwrap_or(0),
        ),
        _ => {
            // Fallback to direct queries if materialized view is empty or fails
            let total_threads: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM threads")
                .fetch_one(pool.get_ref())
                .await
                .unwrap_or(0);

            let open_threads: i64 =
                sqlx::query_scalar("SELECT COUNT(*) FROM threads WHERE is_open = true")
                    .fetch_one(pool.get_ref())
                    .await
                    .unwrap_or(0);

            (total_threads, open_threads, total_threads - open_threads)
        }
    };

    let (total_messages_result, total_notes_result, blocked_users_result) = tokio::join!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM messages").fetch_one(pool.get_ref()),
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM notes").fetch_one(pool.get_ref()),
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM blocked_users")
            .fetch_one(pool.get_ref())
    );

    let total_messages = total_messages_result.unwrap_or(0);
    let total_notes = total_notes_result.unwrap_or(0);
    let blocked_users = blocked_users_result.unwrap_or(0);

    // Optimized response time calculation using new indexes
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
        "#,
    )
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None);

    // Get time-based counts in a single optimized query using new indexes
    let time_counts_result = sqlx::query_as::<_, AnalyticsSummaryTimeCounts>(
        r#"
        SELECT 
            COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE) as threads_today,
            COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE - INTERVAL '7 days') as threads_this_week,
            COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE - INTERVAL '30 days') as threads_this_month
        FROM threads
        WHERE created_at >= CURRENT_DATE - INTERVAL '30 days'
        "#
    )
    .fetch_one(pool.get_ref())
    .await;

    let (threads_today, threads_this_week, threads_this_month) = match time_counts_result {
        Ok(data) => (
            data.threads_today.unwrap_or(0),
            data.threads_this_week.unwrap_or(0),
            data.threads_this_month.unwrap_or(0),
        ),
        Err(_) => {
            // Fallback to individual queries if the optimized query fails
            let threads_today: i64 =
                sqlx::query_scalar("SELECT COUNT(*) FROM threads WHERE created_at >= CURRENT_DATE")
                    .fetch_one(pool.get_ref())
                    .await
                    .unwrap_or(0);

            let threads_this_week: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM threads WHERE created_at >= CURRENT_DATE - INTERVAL '7 days'",
            )
            .fetch_one(pool.get_ref())
            .await
            .unwrap_or(0);

            let threads_this_month: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM threads WHERE created_at >= CURRENT_DATE - INTERVAL '30 days'",
            )
            .fetch_one(pool.get_ref())
            .await
            .unwrap_or(0);

            (threads_today, threads_this_week, threads_this_month)
        }
    };

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

    HttpResponse::Ok().json(overview)
}

#[get("/analytics/thread-volume")]
async fn get_thread_volume(pool: web::Data<PgPool>) -> impl Responder {
    let volume_data_result: Result<Vec<ThreadVolumeData>, sqlx::Error> =
        sqlx::query_as::<_, ThreadVolumeData>(
            r#"
        SELECT 
            date::TEXT as "date!: String",
            thread_count as "count!: i64"
        FROM analytics_summary 
        WHERE date >= CURRENT_DATE - INTERVAL '30 days'
        ORDER BY date DESC
        "#,
        )
        .fetch_all(pool.get_ref())
        .await;

    // Fallback to direct query if materialized view fails or is empty
    let volume_data = match volume_data_result {
        Ok(data) if !data.is_empty() => data,
        _ => {
            // Fallback to original query
            let fallback_result = sqlx::query_as::<_, ThreadVolumeData>(
                r#"
                SELECT 
                    DATE(created_at)::TEXT as "date",
                    COUNT(*) as "count"
                FROM threads 
                WHERE created_at >= CURRENT_DATE - INTERVAL '30 days'
                GROUP BY DATE(created_at)
                ORDER BY DATE(created_at) DESC
                "#,
            )
            .fetch_all(pool.get_ref())
            .await;

            match fallback_result {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Database error fetching thread volume: {}", e);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to fetch thread volume data"
                    }));
                }
            }
        }
    };

    HttpResponse::Ok().json(volume_data)
}

#[get("/analytics/moderator-activity")]
async fn get_moderator_activity(pool: web::Data<PgPool>) -> impl Responder {
    let activity_data_result: Result<Vec<ModeratorActivity>, sqlx::Error> =
        sqlx::query_as::<_, ModeratorActivity>(
            r#"
        SELECT 
            COALESCE(m.author_tag, n.author_tag, 'Unknown') as "moderator_tag",
            COALESCE(message_count, 0) as "message_count",
            COALESCE(note_count, 0) as "note_count",
            0 as "threads_closed"
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
        "#,
        )
        .fetch_all(pool.get_ref())
        .await;

    match activity_data_result {
        Ok(activity_data) => HttpResponse::Ok().json(activity_data),
        Err(e) => {
            eprintln!("Database error fetching moderator activity: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch moderator activity data"
            }))
        }
    }
}

#[get("/analytics/response-times")]
async fn get_response_times(pool: web::Data<PgPool>) -> impl Responder {
    // Execute all response time queries in parallel using the new indexes for better performance
    let (avg_first_response_result, avg_resolution_result, median_first_response_result) = tokio::join!(
        // Average first response time - benefits from new thread_messages covering index
        sqlx::query_scalar::<_, Option<f64>>(
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
            "#,
        ).fetch_optional(pool.get_ref()),

        // Average resolution time - benefits from new created_at indexes
        sqlx::query_scalar::<_, Option<f64>>(
            r#"
            SELECT AVG(EXTRACT(EPOCH FROM (updated_at - created_at)) / 3600.0)
            FROM threads
            WHERE is_open = false 
            AND created_at >= CURRENT_DATE - INTERVAL '30 days'
            AND updated_at IS NOT NULL
            "#,
        ).fetch_optional(pool.get_ref()),

        // Median first response time - benefits from new indexes
        sqlx::query_scalar::<_, Option<f64>>(
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
        ).fetch_optional(pool.get_ref())
    );

    let avg_first_response_hours = avg_first_response_result.unwrap_or(None).flatten();
    let avg_resolution_time_hours = avg_resolution_result.unwrap_or(None).flatten();
    let median_first_response_hours = median_first_response_result.unwrap_or(None).flatten();

    let metrics = ResponseTimeMetrics {
        avg_first_response_hours,
        avg_resolution_time_hours,
        median_first_response_hours,
    };

    HttpResponse::Ok().json(metrics)
}

#[post("/analytics/refresh")]
async fn refresh_analytics(pool: web::Data<PgPool>) -> impl Responder {
    // Refresh the materialized view for up-to-date analytics
    let refresh_result = sqlx::query("SELECT refresh_analytics_summary()")
        .execute(pool.get_ref())
        .await;

    match refresh_result {
        Ok(_) => {
            println!("Analytics materialized view refreshed successfully");
            HttpResponse::Ok().json(serde_json::json!({
                "success": "Analytics data refreshed successfully"
            }))
        }
        Err(e) => {
            eprintln!("Failed to refresh analytics view: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to refresh analytics data"
            }))
        }
    }
}

// Function to automatically refresh analytics in background
pub async fn auto_refresh_analytics(pool: &PgPool) {
    let refresh_result = sqlx::query("SELECT refresh_analytics_summary()")
        .execute(pool)
        .await;

    match refresh_result {
        Ok(_) => println!("Background analytics refresh completed"),
        Err(e) => eprintln!("Background analytics refresh failed: {}", e),
    }
}
