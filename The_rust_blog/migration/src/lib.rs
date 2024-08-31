pub use sea_orm_migration::prelude::*;

mod m20240825_161534_create_user_table;
mod m20240830_162416_create_post_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240825_161534_create_user_table::Migration),
            Box::new(m20240830_162416_create_post_table::Migration),
        ]
    }
}
