use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{drink, addon};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "drinks_addons")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub drink_id: i32,
    #[sea_orm(primary_key)]
    pub addon_id: i32,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::drink::Entity",
        from = "Column::DrinkId",
        to = "super::drink::Column::Id"
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Drink,

    #[sea_orm(
        belongs_to = "super::addon::Entity",
        from = "Column::AddonId",
        to = "super::addon::Column::Id"
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Addon,
}

impl Related<super::addon::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Addon.def()
    }
}

impl Related<super::drink::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Drink.def()
    }
}

impl Linked for Entity {
    type FromEntity = drink::Entity;

    type ToEntity = addon::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::Drink.def().rev(),
            Relation::Addon.def()
        ]
    }
}