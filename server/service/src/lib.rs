mod mutation;
mod query;

pub use mutation::*;
pub use query::*;

pub use sea_orm;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{DatabaseConnection, DbErr};

/// TODO this is incredibly sketchy, read 'https://www.postgresql.org/docs/current/user-manag.html'
pub(crate) async fn check_role(
    connection: &DatabaseConnection,
    user_id: i32,
    role_name: &str,
) -> Result<(), DbErr> {
    use entity::prelude::*;

    let mut roles = UserRole::find()
        .filter(entity::user_role::Column::UserId.eq(user_id))
        .find_also_related(Role)
        .all(connection)
        .await?
        .into_iter()
        .filter_map(|(user, role)| Some((user, role?)));

    if !roles.any(|(_, role)| role.description == role_name) {
        return Err(DbErr::Custom("unauthorized".to_string()));
    }
    Ok(())
}
