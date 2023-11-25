use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::order_status::OrderStatus;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub drink_id: i32,
    pub requester_id: i32,
    pub requested_at: i64,
    pub requested_for: i64,
    pub status: OrderStatus
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::drink::Entity",
        from = "Column::DrinkId",
        to = "super::drink::Column::Id"
    )]
    Drink,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::RequesterId",
        to = "super::user::Column::Id"
    )]
    Requester,
}
