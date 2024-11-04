use super::Mutation;
use entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set};

impl Mutation {
    pub async fn insert_message(
        connection: &DatabaseConnection,
        user_id: i32,
        message_text: String,
    ) -> Result<(), DbErr> {
        let msg = entity::main_message::ActiveModel {
            user_id: Set(user_id),
            message_text: Set(message_text),
            ..Default::default()
        };

        MainMessage::insert(msg).exec(connection).await?;

        Ok(())
    }
}
