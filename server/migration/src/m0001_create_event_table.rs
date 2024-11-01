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
                    .table(Event::Table)
                    .if_not_exists()
                    .col(pk_auto(Event::Id))
                    .col(timestamp_with_time_zone(Event::Date))
                    .col(string(Event::Title))
                    .col(string_null(Event::Description))
                    .col(string_null(Event::Genre))
                    .col(string_null(Event::Url))
                    .col(string_null(Event::Image))
                    .col(string_null(Event::Location))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(EventInterested::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .table(EventInterested::Table)
                            .col(EventInterested::UserId)
                            .col(EventInterested::EventId),
                    )
                    .col(integer(EventInterested::UserId))
                    .col(integer(EventInterested::EventId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(EventInterested::Table, EventInterested::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(EventInterested::Table, EventInterested::UserId)
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
                    .table(EventAttending::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .table(EventAttending::Table)
                            .col(EventAttending::UserId)
                            .col(EventAttending::EventId),
                    )
                    .col(integer(EventAttending::UserId))
                    .col(integer(EventAttending::EventId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(EventAttending::Table, EventAttending::EventId)
                            .to(Event::Table, Event::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(EventAttending::Table, EventInterested::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-event-date")
                    .table(Event::Table)
                    .col(Event::Date)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table(Event::Table)
                    .name("idx-event-date")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(EventAttending::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(EventInterested::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Event {
    Table,
    Id,
    Date,
    Title,
    Description,
    Genre,
    Url,
    Image,
    Location,
}

#[derive(DeriveIden)]
enum EventInterested {
    Table,
    UserId,
    EventId,
}

#[derive(DeriveIden)]
enum EventAttending {
    Table,
    UserId,
    EventId,
}
