use extension::postgres::TypeDropStatement;
use sea_orm::{
    sqlx::Column, ActiveEnum, DbBackend, DeriveActiveEnum, EnumIter, Iterable, Schema,
    StatementBuilder,
};
use sea_orm_migration::{prelude::*, schema::*};
use table::ColumnDef;

use crate::enable_row_level_security;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(WeeklyEvent::Table)
                    .if_not_exists()
                    .col(pk_auto(WeeklyEvent::Id))
                    .col(timestamp_with_time_zone(WeeklyEvent::Date))
                    .col(string(WeeklyEvent::Title))
                    .col(string_null(WeeklyEvent::Description))
                    .col(string(WeeklyEvent::Genre))
                    .col(string_null(WeeklyEvent::Url))
                    .col(string_null(WeeklyEvent::Image))
                    .col(string_null(WeeklyEvent::Location))
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Schema::new(DbBackend::Postgres).create_enum_from_active_enum::<RoleKind>(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Interested::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .table(Interested::Table)
                            .col(Interested::UserId)
                            .col(Interested::EventId),
                    )
                    .col(string(Interested::UserId))
                    .col(integer(Interested::EventId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Interested::Table, Interested::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Interested::Table, Interested::EventId)
                            .to(WeeklyEvent::Table, WeeklyEvent::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Attending::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .table(Attending::Table)
                            .col(Attending::UserId)
                            .col(Attending::EventId),
                    )
                    .col(string(Attending::UserId))
                    .col(integer(Attending::EventId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Attending::Table, Attending::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Attending::Table, Attending::EventId)
                            .to(WeeklyEvent::Table, WeeklyEvent::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-event-date")
                    .table(WeeklyEvent::Table)
                    .col(WeeklyEvent::Date)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        enable_row_level_security(db, "seaql_migrations").await?;
        enable_row_level_security(db, "weekly_event").await?;
        enable_row_level_security(db, "attending").await?;
        enable_row_level_security(db, "interested").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table(WeeklyEvent::Table)
                    .name("idx-event-date")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Attending::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Interested::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(WeeklyEvent::Table).to_owned())
            .await?;
        manager
            .drop_type(TypeDropStatement::new().name(RoleKind::name()).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,

    RoleKind,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role_kind")]
enum RoleKind {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "Audience")]
    Audience,
    #[sea_orm(string_value = "Musician")]
    Musician,
}

#[derive(DeriveIden)]
enum WeeklyEvent {
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
enum Interested {
    Table,
    UserId,
    EventId,
}

#[derive(DeriveIden)]
enum Attending {
    Table,
    UserId,
    EventId,
}
