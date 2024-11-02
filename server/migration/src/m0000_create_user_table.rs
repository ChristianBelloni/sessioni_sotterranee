use sea_orm::{sqlx::Statement, StatementBuilder};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .primary_key(Index::create().table(User::Table).col(User::Id))
                    .col(integer_uniq(User::Id))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .col(pk_auto(Role::Id))
                    .col(string(Role::Description))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .col(integer(UserRole::RoleId))
                    .col(integer(UserRole::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRole::Table, UserRole::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRole::Table, UserRole::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let connection = manager.get_connection();
        connection
            .execute_unprepared("INSERT INTO role (description) VALUES ('admin'),('musician'),('street_jam_organizer'),('audience');")
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum Role {
    Table,
    Id,
    Description,
}

#[derive(DeriveIden)]
enum UserRole {
    Table,
    RoleId,
    UserId,
}
