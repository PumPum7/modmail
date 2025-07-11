//! Contains all the models for the database
use crate::schema::{blocked_users, guild_configs, macros, messages, notes, servers, threads};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Double, Integer, Nullable, Text};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: Uuid,
    pub author_id: String,
    pub author_tag: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub attachments: Option<serde_json::Value>,
    pub guild_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
    pub id: Uuid,
    pub author_id: &'a str,
    pub author_tag: &'a str,
    pub content: &'a str,
    pub created_at: DateTime<Utc>,
    pub attachments: Option<serde_json::Value>,
    pub guild_id: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = threads)]
pub struct Thread {
    pub id: i32,
    pub user_id: String,
    pub thread_id: String,
    pub is_open: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub urgency: Option<String>,
    pub guild_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = threads)]
pub struct NewThread<'a> {
    pub user_id: &'a str,
    pub thread_id: &'a str,
    pub guild_id: &'a str,
    pub urgency: Option<String>,
    pub is_open: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = macros)]
pub struct Macro {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub quick_access: Option<bool>,
    pub guild_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = macros)]
pub struct NewMacro<'a> {
    pub name: &'a str,
    pub content: &'a str,
    pub quick_access: Option<bool>,
    pub guild_id: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = notes)]
pub struct Note {
    pub id: Uuid,
    pub thread_id: Option<i32>,
    pub author_id: String,
    pub author_tag: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub guild_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote<'a> {
    pub id: Uuid,
    pub thread_id: i32,
    pub author_id: &'a str,
    pub author_tag: &'a str,
    pub content: &'a str,
    pub created_at: DateTime<Utc>,
    pub guild_id: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = blocked_users)]
pub struct BlockedUser {
    pub id: i32,
    pub user_id: String,
    pub user_tag: String,
    pub blocked_by: String,
    pub blocked_by_tag: String,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub guild_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = blocked_users)]
pub struct NewBlockedUser<'a> {
    pub user_id: &'a str,
    pub user_tag: &'a str,
    pub blocked_by: &'a str,
    pub blocked_by_tag: &'a str,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub guild_id: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = servers)]
pub struct Server {
    pub id: i32,
    pub guild_id: String,
    pub guild_name: String,
    pub is_premium: bool,
    pub max_threads: Option<i32>,
    pub max_macros: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = servers)]
pub struct NewServer<'a> {
    pub guild_id: &'a str,
    pub guild_name: &'a str,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone, QueryableByName)]
#[diesel(table_name = guild_configs)]
pub struct GuildConfig {
    pub id: i32,
    pub guild_id: String,
    pub modmail_category_id: Option<String>,
    pub log_channel_id: Option<String>,
    pub randomize_names: Option<bool>,
    pub auto_close_hours: Option<i32>,
    pub welcome_message: Option<String>,
    pub moderator_role_ids: Option<Vec<Option<String>>>,
    pub blocked_words: Option<Vec<Option<String>>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = guild_configs)]
pub struct NewGuildConfig<'a> {
    pub guild_id: &'a str,
    pub modmail_category_id: Option<String>,
    pub log_channel_id: Option<String>,
    pub randomize_names: Option<bool>,
    pub auto_close_hours: Option<i32>,
    pub welcome_message: Option<String>,
    pub moderator_role_ids: Option<Vec<String>>,
    pub blocked_words: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = threads)]
pub struct AnalyticsOverview {
    pub total_threads: i64,
    pub open_threads: i64,
    pub closed_threads: i64,
    pub total_messages: i64,
    pub total_notes: i64,
    pub blocked_users: i64,
    pub avg_response_time_hours: Option<f64>,
    pub threads_today: i64,
    pub threads_this_week: i64,
    pub threads_this_month: i64,
}

#[derive(Queryable, Serialize, Deserialize, Clone, Debug, QueryableByName)]
pub struct ThreadVolumeData {
    #[diesel(sql_type = Text)]
    pub date: String,
    #[diesel(sql_type = BigInt)]
    pub count: i64,
}

#[derive(Queryable, Serialize, Deserialize, Clone, Debug, QueryableByName)]
pub struct ModeratorActivity {
    #[diesel(sql_type = Text)]
    pub moderator_tag: String,
    #[diesel(sql_type = BigInt)]
    pub message_count: i64,
    #[diesel(sql_type = BigInt)]
    pub note_count: i64,
    #[diesel(sql_type = Integer)]
    pub threads_closed: i32,
}

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = threads)]
pub struct ResponseTimeMetrics {
    pub avg_first_response_hours: Option<f64>,
    pub avg_resolution_time_hours: Option<f64>,
    pub median_first_response_hours: Option<f64>,
}

// Wrapper structs for scalar SQL queries
#[derive(QueryableByName)]
pub struct CountResult {
    #[diesel(sql_type = BigInt)]
    pub count: i64,
}

#[derive(QueryableByName)]
pub struct DoubleResult {
    #[diesel(sql_type = Nullable<Double>)]
    pub avg: Option<f64>,
}
