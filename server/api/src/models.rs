use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use service::sea_orm::prelude::DateTimeWithTimeZone;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct WeeklyEvent {
    pub id: i32,
    pub date: DateTimeWithTimeZone,
    pub title: String,
    pub description: Option<String>,
    pub genre: String,
    pub url: Option<String>,
    pub image: Option<String>,
    pub location: Option<String>,
}

impl From<entity::weekly_event::Model> for WeeklyEvent {
    fn from(value: entity::weekly_event::Model) -> Self {
        let entity::weekly_event::Model {
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
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub role_kind: RoleKind,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role_kind: RoleKind,
}

impl From<entity::user::Model> for User {
    fn from(value: entity::user::Model) -> Self {
        let entity::user::Model {
            id,
            email,
            username,
            role_kind,
            ..
        } = value;
        Self {
            id,
            username,
            email,
            role_kind: role_kind.into(),
        }
    }
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub enum RoleKind {
    Admin,
    Audience,
    Musician,
}

impl From<entity::sea_orm_active_enums::RoleKind> for RoleKind {
    fn from(value: entity::sea_orm_active_enums::RoleKind) -> Self {
        match value {
            entity::sea_orm_active_enums::RoleKind::Admin => Self::Admin,
            entity::sea_orm_active_enums::RoleKind::Audience => Self::Audience,
            entity::sea_orm_active_enums::RoleKind::Musician => Self::Musician,
        }
    }
}

impl From<RoleKind> for entity::sea_orm_active_enums::RoleKind {
    fn from(value: RoleKind) -> Self {
        match value {
            RoleKind::Admin => entity::sea_orm_active_enums::RoleKind::Admin,
            RoleKind::Audience => entity::sea_orm_active_enums::RoleKind::Audience,
            RoleKind::Musician => entity::sea_orm_active_enums::RoleKind::Musician,
        }
    }
}
