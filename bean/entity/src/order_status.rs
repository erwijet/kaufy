use async_graphql::Enum;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(
    Enum, Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum OrderStatus {
    #[sea_orm(string_value = "Q")]
    Queued,
    #[sea_orm(string_value = "O")]
    Open,
    #[sea_orm(string_value = "D")]
    Done,
    #[sea_orm(string_value = "C")]
    Cancelled,
}
