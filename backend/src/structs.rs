use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateMessage {
    pub author_id: String,
    pub author_tag: String,
    pub content: String,
    pub attachments: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateThread {
    pub user_id: String,
    pub thread_id: String,
}

#[derive(Deserialize)]
pub struct CreateMacro {
    pub name: String,
    pub content: String,
    pub quick_access: Option<bool>,
}

#[derive(Deserialize)]
pub struct CreateNote {
    pub author_id: String,
    pub author_tag: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CreateBlockedUser {
    pub user_id: String,
    pub user_tag: String,
    pub blocked_by: String,
    pub blocked_by_tag: String,
    pub reason: Option<String>,
}

#[derive(Deserialize)]
pub struct CloseThread {
    pub closed_by_id: String,
    pub closed_by_tag: String,
}
