use entity::sea_orm::{DbBackend, EntityTrait, Schema};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmts = vec![
            get_create_stmt_for_trait(entity::order::Entity),
        ];

        for stmt in stmts {
            manager.create_table(stmt).await?
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmts = vec![
            get_drop_stmt_for_trait(entity::order::Entity),
        ];

        for stmt in stmts {
            manager.drop_table(stmt).await?
        }

        Ok(())
    }
}

fn get_create_stmt_for_trait<E: EntityTrait>(entity: E) -> TableCreateStatement {
    let schema = Schema::new(DbBackend::Sqlite);

    schema
        .create_table_from_entity(entity)
        .if_not_exists()
        .take()
}

fn get_drop_stmt_for_trait<E: EntityTrait>(entity: E) -> TableDropStatement {
    Table::drop().table(entity).if_exists().take()
}
