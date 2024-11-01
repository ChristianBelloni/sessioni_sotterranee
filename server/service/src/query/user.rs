use super::Query;
use entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ModelTrait};

impl Query {
    pub async fn get_user(
        connection: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Option<(entity::user::Model, Vec<entity::role::Model>)>, DbErr> {
        let Some(results) = User::find_by_id(user_id).one(connection).await? else {
            return Ok(None);
        };
        let roles = results.find_related(Role).all(connection).await?;

        Ok(Some((results, roles)))
    }
}
