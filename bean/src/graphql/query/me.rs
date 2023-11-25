use std::{ops::Deref, vec};

use crate::{db::Database, graphql::schema::OwnerID};
use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
use entity::{
    acl::{AclRole, Roleset},
    async_graphql, drink, order,
    sea_orm::{ColumnTrait, EntityTrait, QueryFilter},
    user,
};

use super::{drink::DrinkReturn, order::OrderReturn};

#[derive(SimpleObject)]
#[graphql(complex, concrete(name = "Me", params()))]
pub struct MeReturn {
    user: user::Model,
}

impl From<user::Model> for MeReturn {
    fn from(value: user::Model) -> Self {
        Self { user: value }
    }
}

#[ComplexObject]
impl MeReturn {
    pub async fn drinks(&self, ctx: &Context<'_>) -> Result<Vec<DrinkReturn>> {
        let db = ctx.data::<Database>().unwrap();
        let owner_id = ctx.data::<OwnerID>().unwrap().clone();

        Ok(entity::drink::Entity::find()
            .filter(drink::Column::OwnerId.eq(owner_id))
            .all(db)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub async fn orders(&self, ctx: &Context<'_>) -> Result<Vec<OrderReturn>> {
        let db = ctx.data::<Database>().unwrap();
        let owner_id = ctx.data::<OwnerID>().unwrap().clone();

        Ok(entity::order::Entity::find()
            .filter(order::Column::RequesterId.eq(owner_id))
            .all(db)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub async fn roles(&self, ctx: &Context<'_>) -> Result<Vec<AclRole>> {
        Ok(Roleset::from(self.user.roleset).0)
    }
}

#[derive(Default)]
pub struct MeQuery;

#[Object]
impl MeQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<MeReturn> {
        let db = ctx.data::<Database>().unwrap();
        let owner_id = ctx.data::<OwnerID>().unwrap();

        Ok(user::Entity::find_by_id(*owner_id)
            .one(db)
            .await?
            .unwrap()
            .into())
    }
}
