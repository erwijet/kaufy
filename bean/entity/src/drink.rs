use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

use crate::impl_std_ops;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "drinks")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[graphql(skip)]
    pub name: String,
    #[graphql(skip)]
    pub temp_id: i32,
    #[graphql(skip)]
    pub base_id: i32,
    #[graphql(skip)]
    pub owner_id: i32,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::temperature::Entity",
        from = "Column::TempId",
        to = "super::temperature::Column::Id"
    )]
    Temperature,

    #[sea_orm(
        belongs_to = "super::base::Entity",
        from = "Column::BaseId",
        to = "super::base::Column::Id"
    )]
    Base,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id"
    )]
    Owner,
}

impl_std_ops!(Entity);
