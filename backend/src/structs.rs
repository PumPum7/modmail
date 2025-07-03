use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateMessage {
    pub author_id: String,
    pub author_tag: String,
    pub content: String,
    pub attachments: Option<serde_json::Value>,
    pub guild_id: String,
}

#[derive(Deserialize)]
pub struct CreateThread {
    pub user_id: String,
    pub thread_id: String,
    pub guild_id: String,
    pub urgency: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateMacro {
    pub name: String,
    pub content: String,
    pub quick_access: Option<bool>,
    pub guild_id: String,
}

#[derive(Deserialize)]
pub struct CreateNote {
    pub author_id: String,
    pub author_tag: String,
    pub content: String,
    pub guild_id: String,
}

#[derive(Deserialize)]
pub struct CreateBlockedUser {
    pub user_id: String,
    pub user_tag: String,
    pub blocked_by: String,
    pub blocked_by_tag: String,
    pub reason: Option<String>,
    pub guild_id: String,
}

#[derive(Deserialize)]
pub struct CloseThread {
    pub closed_by_id: String,
    pub closed_by_tag: String,
}

#[derive(Deserialize)]
pub struct UpdateThreadUrgency {
    pub urgency: String,
}

// Structs for servers
#[derive(Deserialize)]
pub struct CreateServer {
    pub guild_id: String,
    pub guild_name: String,
}

#[derive(Deserialize)]
pub struct UpdateServer {
    pub guild_name: Option<String>,
    pub is_premium: Option<bool>,
    pub max_threads: Option<i32>,
    pub max_macros: Option<i32>,
}

#[derive(Serialize, FromRow)]
pub struct Server {
    pub id: i32,
    pub guild_id: String,
    pub guild_name: String,
    pub is_premium: bool,
    pub max_threads: i32,
    pub max_macros: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Structs for guild configurations
#[derive(Deserialize)]
pub struct CreateGuildConfig {
    pub guild_id: String,
    pub modmail_category_id: Option<String>,
    pub log_channel_id: Option<String>,
    pub randomize_names: Option<bool>,
    pub auto_close_hours: Option<i32>,
    pub welcome_message: Option<String>,
    pub moderator_role_ids: Option<Vec<String>>,
    pub blocked_words: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct UpdateGuildConfig {
    pub modmail_category_id: Option<String>,
    pub log_channel_id: Option<String>,
    pub randomize_names: Option<bool>,
    pub auto_close_hours: Option<i32>,
    pub welcome_message: Option<String>,
    pub moderator_role_ids: Option<Vec<String>>,
    pub blocked_words: Option<Vec<String>>,
}

#[derive(Serialize, FromRow, Deserialize, Clone)]
pub struct GuildConfig {
    #[serde(default)]
    pub id: i32,
    pub guild_id: String,
    pub modmail_category_id: Option<String>,
    pub log_channel_id: Option<String>,
    pub randomize_names: bool,
    pub auto_close_hours: Option<i32>,
    pub welcome_message: Option<String>,
    pub moderator_role_ids: Vec<String>,
    pub blocked_words: Vec<String>,
    #[serde(default)]
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub updated_at: DateTime<Utc>,
}

// Guild validation structs
#[derive(Deserialize)]
pub struct ValidateGuildRequest {
    pub guild_id: String,
    pub guild_name: String,
    pub guild_icon: Option<String>,
    pub user_has_permissions: bool,
}

#[derive(Serialize)]
pub struct ValidatedGuild {
    pub guild_id: String,
    pub guild_name: String,
    pub guild_icon: Option<String>,
    pub has_bot: bool,
    pub has_config: bool,
    pub user_has_permissions: bool,
}
