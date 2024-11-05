use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use service::sea_orm::prelude::DateTimeWithTimeZone;

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug)]
pub struct IdentifyClient {
    pub user_id: i32,
}

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug)]
pub struct SendMainChatMessage {
    pub sender_id: i32,
    pub message_text: String,
    pub date: DateTimeWithTimeZone,
}

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug)]
pub enum MessageSenderUsername {
    #[default]
    Me,
    Other(String),
}

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug)]
pub struct MainChatMessage {
    pub sender: MessageSenderUsername,
    pub message_text: String,
    pub date: DateTimeWithTimeZone,
}

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug)]
pub struct RequestMainChatHistory {
    pub user_id: i32,
}

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug)]
pub struct PublishedMainchatHistory {
    pub user_id: i32,
    pub messages: Vec<MainChatMessage>,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub enum WebSocketMessage {
    IdentifyClient(IdentifyClient),
    SentMainChatMessage(SendMainChatMessage),
    PublishedMainChatMessage(MainChatMessage),
    RequestMainChatHistory(RequestMainChatHistory),
    PublishedMainChatHistory(PublishedMainchatHistory),
}
