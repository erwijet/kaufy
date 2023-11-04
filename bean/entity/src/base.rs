use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

use crate::impl_std_ops;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "bases")]
#[graphql(concrete(name = "Base", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl_std_ops!(Entity);