use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use service::sea_orm::prelude::DateTimeWithTimeZone;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub roles: Vec<Role>,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub description: String,
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
