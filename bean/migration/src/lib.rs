pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20231028_221158_create_table;
mod m20231030_031606_temps_table;
mod m20231030_033336_use_numbers_in_drink_addons;
mod m20231111_004759_orders;
mod m20231123_200526_roles;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20231028_221158_create_table::Migration),
            Box::new(m20231030_031606_temps_table::Migration),
            Box::new(m20231030_033336_use_numbers_in_drink_addons::Migration),
            Box::new(m20231111_004759_orders::Migration),
            Box::new(m20231123_200526_roles::Migration),
        ]
    }
}
