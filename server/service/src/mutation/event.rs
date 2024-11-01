use entity::event;
use sea_orm::{
    entity::*,
    sqlx::types::chrono::{DateTime, FixedOffset},
    DatabaseConnection, DbErr, Set,
};

use super::Mutation;

impl Mutation {
    pub async fn insert_event(
        title: &str,
        date: DateTime<FixedOffset>,
        description: Option<&str>,
        genre: &str,
        url: Option<&str>,
        image: Option<&str>,
        location: Option<&str>,
        connection: &DatabaseConnection,
    ) -> Result<event::Model, DbErr> {
        let event = event::ActiveModel {
            title: Set(title.to_owned()),
            date: Set(date),
            description: Set(description.map(ToString::to_string)),
            genre: Set(Some(genre.to_owned())),
            url: Set(url.map(ToString::to_string)),
            image: Set(image.map(ToString::to_string)),
            location: Set(location.map(ToString::to_string)),
            ..Default::default()
        };
        event.insert(connection).await
    }

    pub async fn delete_event(event_id: i32, connection: &DatabaseConnection) -> Result<(), DbErr> {
        event::ActiveModel {
            id: Set(event_id),
            ..Default::default()
        }
        .delete(connection)
        .await?;

        Ok(())
    }
}