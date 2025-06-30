use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::uuid::Uuid, Pool, Postgres};
use std::time::Duration;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub author_id: String,
    pub author_tag: String,
    pub content: String,
    pub attachments: serde_json::Value,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Thread {
    pub id: i32,
    pub user_id: String,
    pub thread_id: String,
    pub is_open: bool,
    pub urgency: String,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Macro {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub quick_access: bool,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub thread_id: i32,
    pub author_id: String,
    pub author_tag: String,
    pub content: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct BlockedUser {
    pub id: i32,
    pub user_id: String,
    pub user_tag: String,
    pub blocked_by: String,
    pub blocked_by_tag: String,
    pub reason: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn connect(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600)) // 10 minutes
        .max_lifetime(Duration::from_secs(1800)) // 30 minutes
        .test_before_acquire(true)
        .connect(database_url)
        .await
}
