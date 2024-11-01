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
                    .table(StreetJam::Table)
                    .if_not_exists()
                    .col(pk_auto(StreetJam::Id))
                    .col(integer(StreetJam::CreatorId))
                    .col(timestamp_with_time_zone(StreetJam::Date))
                    .col(string(StreetJam::Title))
                    .col(string_null(StreetJam::Description))
                    .col(string(StreetJam::Genre))
                    .col(string_null(StreetJam::Location))
                    .foreign_key(
                        ForeignKey::create()
                            .from(StreetJam::Table, StreetJam::CreatorId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(StreetJamMessage::Table)
                    .col(pk_auto(StreetJamMessage::Id))
                    .col(integer(StreetJamMessage::StreetJamId))
                    .col(integer(StreetJamMessage::UserId))
                    .col(timestamp_with_time_zone(StreetJamMessage::Date))
                    .col(text(StreetJamMessage::MessageText))
                    .foreign_key(
                        ForeignKey::create()
                            .from(StreetJamMessage::Table, StreetJamMessage::StreetJamId)
                            .to(StreetJam::Table, StreetJam::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StreetJamMessage::Table, StreetJamMessage::UserId)
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
            .drop_table(Table::drop().table(StreetJamMessage::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(StreetJam::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum StreetJam {
    Table,
    Id,
    CreatorId,
    Date,
    Title,
    Description,
    Genre,
    Location,
}

#[derive(DeriveIden)]
enum StreetJamMessage {
    Table,
    Id,
    StreetJamId,
    UserId,
    Date,
    MessageText,
}
