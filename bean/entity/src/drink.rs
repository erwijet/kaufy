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

impl Related<super::addon::Entity> for Entity {
    fn to() -> RelationDef {
        super::drink_addon::Relation::Addon.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::drink_addon::Relation::Addon.def().rev())
    }
}

impl Related<super::temperature::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Temperature.def()
    }
}

impl Related<super::base::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Base.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl_std_ops!(Entity);
