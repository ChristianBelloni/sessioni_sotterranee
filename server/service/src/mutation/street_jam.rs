use entity::prelude::*;
use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter, Set, TryIntoModel,
};

use crate::check_role;

use super::Mutation;

impl Mutation {
    pub async fn insert_street_jam(
        connection: &DatabaseConnection,
        creator_id: i32,
        date: DateTimeWithTimeZone,
        title: String,
        description: Option<String>,
        genre: Option<String>,
        location: Option<String>,
    ) -> Result<entity::street_jam::Model, DbErr> {
        User::find_by_id(creator_id)
            .one(connection)
            .await?
            .ok_or(DbErr::RecordNotFound("user not found".to_string()))?;

        check_role(connection, creator_id, "street_jam_organizer").await?;

        let new_jam = entity::street_jam::ActiveModel {
            creator_id: Set(creator_id),
            date: Set(date),
            title: Set(title),
            genre: Set(genre),
            description: Set(description),
            location: Set(location),
            ..Default::default()
        };
        let new_jam = StreetJam::insert(new_jam)
            .exec_with_returning(connection)
            .await?;
        Ok(new_jam)
    }

    pub async fn update_street_jam(
        connection: &DatabaseConnection,
        creator_id: i32,
        street_jam_id: i32,
        date: Option<DateTimeWithTimeZone>,
        title: Option<String>,
        description: Option<Option<String>>,
        genre: Option<Option<String>>,
        location: Option<Option<String>>,
    ) -> Result<entity::street_jam::Model, DbErr> {
        let updated = StreetJam::find_by_id(street_jam_id)
            .filter(entity::street_jam::Column::CreatorId.eq(creator_id))
            .one(connection)
            .await?
            .ok_or(DbErr::RecordNotFound("jam not found".to_string()))?;

        let mut updated = updated.into_active_model();

        if let Some(date) = date {
            updated.date = Set(date);
        }

        if let Some(title) = title {
            updated.title = Set(title)
        }

        if let Some(description) = description {
            updated.description = Set(description);
        }

        if let Some(genre) = genre {
            updated.genre = Set(genre);
        }

        if let Some(location) = location {
            updated.location = Set(location)
        }

        StreetJam::update(updated.clone())
            .filter(entity::street_jam::Column::Id.eq(street_jam_id))
            .exec(connection)
            .await?;

        Ok(updated.try_into_model()?)
    }
}
