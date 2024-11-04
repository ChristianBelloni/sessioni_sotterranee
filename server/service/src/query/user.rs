use super::Query;
use entity::{prelude::*, user, user_role};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter, Set};

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

    pub async fn get_user_from_logto_id(
        connection: &DatabaseConnection,
        user_id: String,
    ) -> Result<Option<(entity::user::Model, Vec<entity::role::Model>)>, DbErr> {
        tracing::info!("get user by {user_id}");
        let Some(results) = User::find()
            .filter(user::Column::LogToId.eq(user_id))
            .one(connection)
            .await?
        else {
            return Ok(None);
        };
        let roles = results.find_related(Role).all(connection).await?;

        Ok(Some((results, roles)))
    }
}
