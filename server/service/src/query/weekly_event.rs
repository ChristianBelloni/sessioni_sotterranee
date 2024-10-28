use super::Query;
use entity::{prelude::*, weekly_event};
use sea_orm::{
    sqlx::types::chrono::Utc, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Select, SelectTwo,
};

trait WeeklyEventQueryExt {
    fn after_now(self) -> Self;
    fn date_asc(self) -> Self;
}

impl WeeklyEventQueryExt for Select<WeeklyEvent> {
    fn after_now(self) -> Self {
        self.filter(weekly_event::Column::Date.gt(Utc::now()))
    }
    fn date_asc(self) -> Self {
        self.order_by(entity::weekly_event::Column::Date, sea_orm::Order::Asc)
    }
}

impl<T: EntityTrait> WeeklyEventQueryExt for SelectTwo<T, WeeklyEvent> {
    fn date_asc(self) -> Self {
        self.filter(weekly_event::Column::Date.gt(Utc::now()))
    }
    fn after_now(self) -> Self {
        self.order_by(entity::weekly_event::Column::Date, sea_orm::Order::Asc)
    }
}

impl Query {
    pub async fn weekly_event_by_id(
        connection: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<entity::weekly_event::Model>, DbErr> {
        let value = entity::prelude::WeeklyEvent::find_by_id(id)
            .one(connection)
            .await?;
        Ok(value)
    }

    pub async fn get_next_evnts(
        connection: &DatabaseConnection,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<entity::weekly_event::Model>, DbErr> {
        WeeklyEvent::find()
            .after_now()
            .date_asc()
            .offset(offset)
            .limit(limit)
            .all(connection)
            .await
    }

    pub async fn get_evnts_for_user(
        connection: &DatabaseConnection,
        user_id: i32,
        offset: u64,
        limit: u64,
    ) -> Result<
        (
            Vec<entity::weekly_event::Model>,
            Vec<entity::weekly_event::Model>,
        ),
        DbErr,
    > {
        let interested = Interested::find()
            .filter(entity::interested::Column::UserId.eq(user_id))
            .find_also_related(WeeklyEvent)
            .after_now()
            .date_asc()
            .offset(offset)
            .limit(limit)
            .all(connection)
            .await?
            .into_iter()
            .filter_map(|(_, a)| a)
            .collect();

        let attending = Attending::find()
            .filter(entity::attending::Column::UserId.eq(user_id))
            .find_also_related(WeeklyEvent)
            .after_now()
            .date_asc()
            .all(connection)
            .await?
            .into_iter()
            .filter_map(|(_, a)| a)
            .collect();

        Ok((interested, attending))
    }
}
