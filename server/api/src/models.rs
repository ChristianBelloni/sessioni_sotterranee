use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use service::sea_orm::prelude::DateTimeWithTimeZone;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub log_to_id: String,
    pub roles: Vec<Role>,
    pub username: String,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub description: String,
}

impl From<entity::role::Model> for Role {
    fn from(value: entity::role::Model) -> Self {
        Self {
            id: value.id,
            description: value.description,
        }
    }
}

impl From<(entity::user::Model, Vec<entity::role::Model>)> for User {
    fn from((user, roles): (entity::user::Model, Vec<entity::role::Model>)) -> Self {
        Self {
            id: user.id,
            log_to_id: user.log_to_id,
            roles: roles.into_iter().map(Role::from).collect(),
            username: user.username,
        }
    }
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct CompleteRegistrationRequest {
    pub username: String,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub date: DateTimeWithTimeZone,
    pub title: String,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub url: Option<String>,
    pub image: Option<String>,
    pub location: Option<String>,
}

impl From<entity::event::Model> for Event {
    fn from(value: entity::event::Model) -> Self {
        let entity::event::Model {
            id,
            date,
            title,
            description,
            genre,
            url,
            image,
            location,
        } = value;
        Self {
            id,
            date,
            title,
            description,
            genre,
            url,
            image,
            location,
        }
    }
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct EventsForUser {
    pub attending: Vec<Event>,
    pub interested: Vec<Event>,
}

impl From<(Vec<entity::event::Model>, Vec<entity::event::Model>)> for EventsForUser {
    fn from(
        (interested, attending): (Vec<entity::event::Model>, Vec<entity::event::Model>),
    ) -> Self {
        Self {
            interested: interested.into_iter().map(Event::from).collect(),
            attending: attending.into_iter().map(Event::from).collect(),
        }
    }
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct Message {
    pub message_text: String,
    pub date: DateTimeWithTimeZone,
    pub sender_username: String,
}

impl From<(entity::user::Model, entity::main_message::Model)> for Message {
    fn from((user, message): (entity::user::Model, entity::main_message::Model)) -> Self {
        Message {
            message_text: message.message_text,
            date: message.date,
            sender_username: user.username,
        }
    }
}
