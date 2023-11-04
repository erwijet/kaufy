use std::ops::Deref;

use entity::sea_orm::{self, ConnectionTrait};
use sea_orm::DatabaseConnection;

pub struct Database(DatabaseConnection);

impl std::ops::Deref for Database {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Database {
    pub async fn new() -> Self {
        let connection = sea_orm::Database::connect(std::env::var("DATABASE_URL").unwrap())
            .await
            .expect("Could not connect to database");

        Self(connection)
    }
}

// duck type Database to `sea_orm::DatabaseConnection`

impl ConnectionTrait for Database {
    fn execute<'life0, 'async_trait>(
        &'life0 self,
        stmt: sea_orm::Statement,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<sea_orm::ExecResult, sea_orm::DbErr>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.deref().execute(stmt)
    }

    fn execute_unprepared<'life0, 'life1, 'async_trait>(
        &'life0 self,
        sql: &'life1 str,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<sea_orm::ExecResult, sea_orm::DbErr>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        self.deref().execute_unprepared(sql)
    }

    fn get_database_backend(&self) -> sea_orm::DbBackend {
        self.deref().get_database_backend()
    }

    fn is_mock_connection(&self) -> bool {
        self.deref().is_mock_connection()
    }

    fn query_all<'life0, 'async_trait>(
        &'life0 self,
        stmt: sea_orm::Statement,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Vec<sea_orm::QueryResult>, sea_orm::DbErr>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.deref().query_all(stmt)
    }

    fn query_one<'life0, 'async_trait>(
        &'life0 self,
        stmt: sea_orm::Statement,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Option<sea_orm::QueryResult>, sea_orm::DbErr>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.deref().query_one(stmt)
    }

    fn support_returning(&self) -> bool {
        self.deref().support_returning()
    }
}
