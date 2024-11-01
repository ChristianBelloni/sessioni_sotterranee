pub use sea_orm_migration::prelude::*;

mod m0000_create_user_table;
mod m0001_create_event_table;
mod m0002_create_street_jam_table;
mod m0003_create_main_chat_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m0000_create_user_table::Migration),
            Box::new(m0001_create_event_table::Migration),
            Box::new(m0002_create_street_jam_table::Migration),
            Box::new(m0003_create_main_chat_table::Migration),
        ]
    }
}

#[allow(unused)]
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
