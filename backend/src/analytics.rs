use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{AnalyticsOverview, ModeratorActivity, ResponseTimeMetrics, ThreadVolumeData};
use crate::schema::{blocked_users, messages, notes, threads};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use diesel::prelude::*;
use diesel::sql_types::Text;

pub fn analytics_routes(db_pool: DbPool) -> Router {
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

async fn get_analytics_overview(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;

    let total_threads: i64 = threads::table
        .filter(threads::guild_id.eq(&guild_id_path))
        .count()
        .get_result(&mut conn)?;

    let open_threads: i64 = threads::table
        .filter(
            threads::is_open
                .eq(true)
                .and(threads::guild_id.eq(&guild_id_path)),
        )
        .count()
        .get_result(&mut conn)?;

    let closed_threads = total_threads - open_threads;

    let threads_today = threads::table
        .filter(threads::guild_id.eq(&guild_id_path))
        .filter(threads::created_at.ge(Utc::now().date_naive()))
        .count()
        .get_result(&mut conn)?;

    let threads_this_week: i64 = diesel::sql_query(
        "SELECT COUNT(*) FROM threads WHERE guild_id = $1 AND created_at >= CURRENT_DATE - INTERVAL '7 days'",
    )
    .bind::<Text, _>(&guild_id_path)
    .get_result(&mut conn)?;

    let threads_this_month: i64 = diesel::sql_query(
        "SELECT COUNT(*) FROM threads WHERE guild_id = $1 AND created_at >= CURRENT_DATE - INTERVAL '30 days'",
    )
    .bind::<Text, _>(&guild_id_path)
    .get_result(&mut conn)?;

    let avg_response_time_hours: Option<f64> = diesel::sql_query(
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
                AND m.author_id != t.user_id
            GROUP BY t.id, t.created_at
        )
        SELECT AVG(EXTRACT(EPOCH FROM (first_message - thread_created)) / 3600.0)
        FROM first_responses
        WHERE first_message > thread_created
        "#,
    )
    .bind::<Text, _>(&guild_id_path)
    .get_result(&mut conn)?;

    let total_messages: i64 = messages::table
        .filter(messages::guild_id.eq(&guild_id_path))
        .count()
        .get_result(&mut conn)?;

    let total_notes: i64 = notes::table
        .filter(notes::guild_id.eq(&guild_id_path))
        .count()
        .get_result(&mut conn)?;

    let blocked_users_count: i64 = blocked_users::table
        .filter(blocked_users::guild_id.eq(&guild_id_path))
        .count()
        .get_result(&mut conn)?;

    let overview = AnalyticsOverview {
        total_threads,
        open_threads,
        closed_threads,
        total_messages,
        total_notes,
        blocked_users: blocked_users_count,
        avg_response_time_hours,
        threads_today,
        threads_this_week,
        threads_this_month,
    };

    Ok((StatusCode::OK, Json(overview)))
}

async fn get_thread_volume(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let volume_data = diesel::sql_query(
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
    .bind::<Text, _>(guild_id_path)
    .load::<ThreadVolumeData>(&mut conn)?;

    Ok((StatusCode::OK, Json(volume_data)))
}

async fn get_moderator_activity(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let activity_data = diesel::sql_query(
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
    .bind::<Text, _>(guild_id_path)
    .load::<ModeratorActivity>(&mut conn)?;

    Ok((StatusCode::OK, Json(activity_data)))
}

async fn get_response_times(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let avg_first_response_hours: Option<f64> = diesel::sql_query(
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
                AND m.author_id != t.user_id
            GROUP BY t.id, t.created_at
        )
        SELECT AVG(EXTRACT(EPOCH FROM (first_response - thread_created)) / 3600.0)
        FROM first_responses
        WHERE first_response > thread_created
        "#,
    )
    .bind::<Text, _>(&guild_id_path)
    .get_result(&mut conn)?;

    let avg_resolution_time_hours: Option<f64> = diesel::sql_query(
        r#"
        SELECT AVG(EXTRACT(EPOCH FROM (updated_at - created_at)) / 3600.0)
        FROM threads
        WHERE is_open = false 
            AND guild_id = $1 
            AND updated_at >= CURRENT_DATE - INTERVAL '30 days'
        "#,
    )
    .bind::<Text, _>(&guild_id_path)
    .get_result(&mut conn)?;

    let median_first_response_hours: Option<f64> = diesel::sql_query(
        r#"
        WITH first_responses AS (
            SELECT 
                t.created_at as thread_created,
                MIN(m.created_at) as first_response
            FROM threads t
            INNER JOIN thread_messages tm ON t.id = tm.thread_id
            INNER JOIN messages m ON tm.message_id = m.id
            WHERE t.guild_id = $1 
                AND t.created_at >= CURRENT_DATE - INTERVAL '30 days'
                AND m.author_id != t.user_id
            GROUP BY t.id, t.created_at
        )
        SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY EXTRACT(EPOCH FROM (first_response - thread_created)) / 3600.0)
        FROM first_responses
        WHERE first_response > thread_created
        "#,
    )
    .bind::<Text, _>(guild_id_path)
    .get_result(&mut conn)?;

    let metrics = ResponseTimeMetrics {
        avg_first_response_hours,
        avg_resolution_time_hours,
        median_first_response_hours,
    };

    Ok((StatusCode::OK, Json(metrics)))
}

async fn refresh_analytics(
    State(_pool): State<DbPool>,
    Path(_guild_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // This could trigger a background job to update materialized views or a cache
    Ok((StatusCode::OK, "Analytics refresh job started"))
}
