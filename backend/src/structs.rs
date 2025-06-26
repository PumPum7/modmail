use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateMessage {
    pub author_id: String,
    pub author_tag: String,
    pub content: String,
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
}
