use sea_orm::DatabaseConnection;
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_weekly_event_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20220101_000001_create_weekly_event_table::Migration,
        )]
    }
}

async fn enable_row_level_security(
    db: &SchemaManagerConnection<'_>,
    table: &str,
) -> Result<(), DbErr> {
    db.execute_unprepared(&format!(
        "ALTER TABLE \"{table}\" ENABLE ROW LEVEL SECURITY"
    ))
    .await?;

    db.execute_unprepared(&format!("ALTER TABLE \"{table}\" FORCE ROW LEVEL SECURITY"))
        .await?;

    Ok(())
}
