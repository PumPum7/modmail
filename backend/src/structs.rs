use diesel::prelude::{AsChangeset, Identifiable};
use serde::{Deserialize, Serialize};

use crate::schema::guild_configs;

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

// Structs for guild configurations
#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = guild_configs)]
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
