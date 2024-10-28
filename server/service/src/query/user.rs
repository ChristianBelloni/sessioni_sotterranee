use super::Query;
use entity::{prelude::*, sea_orm_active_enums::RoleKind};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QuerySelect, Select,
};

trait UserExt {
    fn filter_by_role(self, role: RoleKind) -> Self;
}

impl UserExt for Select<User> {
    fn filter_by_role(self, role: RoleKind) -> Self {
        self.filter(entity::user::Column::RoleKind.eq(role))
    }
}

impl Query {
    pub async fn user_by_id(
        connection: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<entity::user::Model>, DbErr> {
        let res = User::find_by_id(id).one(connection).await?;

        Ok(res)
    }

    pub async fn users_by_role_kind(
        connection: &DatabaseConnection,
        kind: RoleKind,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<entity::user::Model>, DbErr> {
        let res = User::find()
            .filter_by_role(kind)
            .offset(Some(offset))
            .limit(Some(limit))
            .all(connection)
            .await?
            .into_iter()
            .collect();

        Ok(res)
    }
}
