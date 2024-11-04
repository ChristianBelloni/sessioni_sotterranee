use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use service::sea_orm::prelude::DateTimeWithTimeZone;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct IdentifyClient {
    pub user_id: i32,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct SendMainChatMessage {
    pub sender_id: i32,
    pub message_text: String,
    pub date: DateTimeWithTimeZone,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub enum MessageSenderUsername {
    Me,
    Other(String),
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct MainChatMessage {
    pub sender: MessageSenderUsername,
    pub message_text: String,
    pub date: DateTimeWithTimeZone,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct RequestMainChatHistory {
    pub user_id: i32,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct PublishedMainchatHistory {
    pub user_id: i32,
    pub messages: Vec<MainChatMessage>,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub enum WebSocketMessage {
    IdentifyClient(IdentifyClient),
    SentMainChatMessage(SendMainChatMessage),
    PublishedMainChatMessage(MainChatMessage),
    RequestMainChatHistory(RequestMainChatHistory),
    PublishedMainChatHistory(PublishedMainchatHistory),
}
