use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::uuid::Uuid, Pool, Postgres};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Message {
    #[serde(with = "uuid::serde::compact")]
    pub id: Uuid,
    pub author_id: String,
    pub author_tag: String,
    pub content: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Thread {
    pub id: i32,
    pub user_id: String,
    pub thread_id: String,
    pub is_open: bool,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Macro {
    pub id: i32,
    pub name: String,
    pub content: String,
}

pub async fn connect(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
