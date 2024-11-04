use entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryOrder, QuerySelect};

impl super::Query {
    pub async fn get_messages(
        connection: &DatabaseConnection,
        offset: u64,
        limit: u64,
    ) -> Result<Vec<entity::main_message::Model>, DbErr> {
        MainMessage::find()
            .order_by_desc(entity::main_message::Column::Date)
            .offset(offset)
            .limit(limit)
            .all(connection)
            .await
    }
}
