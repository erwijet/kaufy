use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

use crate::impl_std_ops;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "addons")]
#[graphql(concrete(name = "Addon", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::drink::Entity> for Entity {
    fn to() -> RelationDef {
        super::drink_addon::Relation::Drink.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::drink_addon::Relation::Drink.def().rev())
    }
}

impl_std_ops!(Entity);