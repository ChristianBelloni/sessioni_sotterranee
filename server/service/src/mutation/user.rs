use entity::{sea_orm_active_enums::RoleKind, user};
use sea_orm::{entity::*, DatabaseConnection, DbErr, Set};

use super::Mutation;

impl Mutation {
    pub async fn insert_user(
        connection: &DatabaseConnection,
        username: &str,
        email: &str,
        role: RoleKind,
    ) -> Result<user::Model, DbErr> {
        user::ActiveModel {
            username: Set(username.to_owned()),
            email: Set(email.to_owned()),
            role_kind: Set(role),
            ..Default::default()
        }
        .insert(connection)
        .await
    }

    pub async fn delete_user(connection: &DatabaseConnection, user_id: i32) -> Result<(), DbErr> {
        user::ActiveModel {
            id: Set(user_id),
            ..Default::default()
        }
        .delete(connection)
        .await?;

        Ok(())
    }
}
