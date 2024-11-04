use super::Mutation;
use entity::{prelude::*, user, user_role};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, QueryFilter, Set,
};

impl Mutation {
    pub async fn insert_user(
        connection: &DatabaseConnection,
        log_to_id: String,
        role_ids: Vec<i32>,
    ) -> Result<(entity::user::Model, Vec<entity::role::Model>), DbErr> {
        let user = user::ActiveModel {
            log_to_id: Set(log_to_id),
            username: Set("".to_string()),
            ..Default::default()
        };
        let user = User::insert(user).exec_with_returning(connection).await?;
        let mut roles = Vec::new();
        for role in role_ids {
            if let Some(role) = Role::find_by_id(role).one(connection).await? {
                let user_role = user_role::ActiveModel {
                    user_id: Set(user.id),
                    role_id: Set(role.id),
                };
                UserRole::insert(user_role).exec(connection).await?;
                roles.push(role);
            }
        }

        Ok((user, roles))
    }

    pub async fn delete_user_by_logto_id(
        connection: &DatabaseConnection,
        log_to_id: String,
    ) -> Result<(), DbErr> {
        User::delete_many()
            .filter(entity::user::Column::LogToId.eq(log_to_id))
            .exec(connection)
            .await?;

        Ok(())
    }

    pub async fn add_role_to_user(
        connection: &DatabaseConnection,
        user_id: i32,
        role: i32,
    ) -> Result<(), DbErr> {
        let role = Role::find_by_id(role)
            .one(connection)
            .await?
            .ok_or(DbErr::RecordNotFound("role not found".to_string()))?;

        let user_role = user_role::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role.id),
        };
        UserRole::insert(user_role).exec(connection).await?;
        Ok(())
    }

    pub async fn update_username(
        connection: &DatabaseConnection,
        user_id: i32,
        username: String,
    ) -> Result<(), DbErr> {
        let user = User::find_by_id(user_id)
            .one(connection)
            .await?
            .ok_or(DbErr::RecordNotUpdated)?;
        let mut user_active = user.into_active_model();
        user_active.username = Set(username);

        User::update(user_active)
            .filter(entity::user::Column::Id.eq(user_id))
            .exec(connection)
            .await?;

        Ok(())
    }
}
