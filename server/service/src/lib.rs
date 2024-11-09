mod mutation;
mod query;

use std::time::Duration;

use chrono::{Days, SubsecRound};
pub use mutation::*;
pub use query::*;

pub use sea_orm;
use sea_orm::sqlx::types::chrono::{FixedOffset, Utc};
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

pub async fn insert_test_data(connection: &DatabaseConnection) {
    for i in 3..100 {
        let test_event = entity::event::ActiveModel {
            title: sea_orm::ActiveValue::Set(format!("test event {i}")),
            date: sea_orm::ActiveValue::Set(
                Utc::now()
                    .checked_add_days(Days::new(i))
                    .unwrap()
                    .with_timezone(&FixedOffset::east_opt(0).unwrap())
                    .round_subsecs(0),
            ),
            ..Default::default()
        };

        _ = entity::prelude::Event::insert(test_event)
            .exec(connection)
            .await;
    }
}
