use sea_orm_migration::{prelude::*, schema::*};

use crate::m0000_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MainMessage::Table)
                    .col(pk_auto(MainMessage::Id))
                    .col(integer(MainMessage::UserId))
                    .col(timestamp_with_time_zone(MainMessage::Date))
                    .col(text(MainMessage::MessageText))
                    .foreign_key(
                        ForeignKey::create()
                            .from(MainMessage::Table, MainMessage::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MainMessage::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum MainMessage {
    Table,
    Id,
    UserId,
    Date,
    MessageText,
}
