use super::Query;
use entity::{event, prelude::*};
use sea_orm::{
    sqlx::types::chrono::Utc, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Select, SelectTwo,
};

trait EventQueryExt {
    fn after_now(self) -> Self;
    fn date_asc(self) -> Self;
}

impl EventQueryExt for Select<Event> {
    fn after_now(self) -> Self {
        self.filter(event::Column::Date.gt(Utc::now()))
    }
    fn date_asc(self) -> Self {
        self.order_by(entity::event::Column::Date, sea_orm::Order::Asc)
    }
}

impl<T: EntityTrait> EventQueryExt for SelectTwo<T, Event> {
    fn date_asc(self) -> Self {
        self.filter(event::Column::Date.gt(Utc::now()))
    }
    fn after_now(self) -> Self {
        self.order_by(entity::event::Column::Date, sea_orm::Order::Asc)
    }
}

impl Query {
    pub async fn event_by_id(
        connection: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<entity::event::Model>, DbErr> {
        let value = entity::prelude::Event::find_by_id(id)
            .one(connection)
            .await?;
        Ok(value)
    }

    pub async fn get_next_evnts(
        connection: &DatabaseConnection,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<entity::event::Model>, DbErr> {
        Event::find()
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
    ) -> Result<(Vec<entity::event::Model>, Vec<entity::event::Model>), DbErr> {
        let interested = EventInterested::find()
            .filter(entity::event_interested::Column::UserId.eq(user_id))
            .find_also_related(Event)
            .after_now()
            .date_asc()
            .offset(offset)
            .limit(limit)
            .all(connection)
            .await?
            .into_iter()
            .filter_map(|(_, a)| a)
            .collect();

        let attending = EventAttending::find()
            .filter(entity::event_attending::Column::UserId.eq(user_id))
            .find_also_related(Event)
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
