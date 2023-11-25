use std::ops::Deref;

use async_graphql::{ComplexObject, Context, Object, Result};
use entity::async_graphql::{self, SimpleObject};
use entity::order;
use entity::order_status::OrderStatus;
use entity::sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use entity::{drink, user};

use crate::db::Database;
use crate::graphql::schema::OwnerID;

#[derive(SimpleObject)]
#[graphql(complex, concrete(name = "Order", params()))]
pub struct OrderReturn {
    id: i32,
    #[graphql(skip)]
    model: order::Model,
}

impl std::ops::Deref for OrderReturn {
    type Target = order::Model;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl From<order::Model> for OrderReturn {
    fn from(value: order::Model) -> Self {
        OrderReturn {
            id: value.id,
            model: value,
        }
    }
}

#[ComplexObject]
impl OrderReturn {
    async fn drink(&self, ctx: &Context<'_>) -> super::drink::DrinkReturn {
        let db = ctx.data::<Database>().unwrap();
        drink::Entity::find_by_id(self.drink_id)
            .one(db)
            .await
            .unwrap()
            .unwrap()
            .into()
    }

    async fn requester(&self, ctx: &Context<'_>) -> user::Model {
        let db = ctx.data::<Database>().unwrap();
        user::Entity::find_by_id(self.requester_id)
            .one(db)
            .await
            .unwrap()
            .unwrap()
    }

    async fn status(&self) -> OrderStatus {
        self.deref().status
    }

    async fn requested_at(&self) -> i64 {
        self.deref().requested_at
    }

    async fn requested_for(&self) -> i64 {
        self.deref().requested_for
    }
}

#[derive(Default)]
pub struct OrderQuery;

#[Object]
impl OrderQuery {
    pub async fn order(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The ID of the order")] id: i32,
    ) -> Result<Option<OrderReturn>> {
        let db = ctx.data::<Database>().unwrap();
        let order = order::Entity::find_by_id(id).one(db).await?;

        Ok(order.map(Into::into))
    }

    pub async fn orders(&self, ctx: &Context<'_>) -> Result<Vec<OrderReturn>> {
        let db = ctx.data::<Database>().unwrap();

        let orders = order::Entity::find().all(db).await?;
        Ok(orders.into_iter().map(Into::into).collect())
    }

    pub async fn my_orders(&self, ctx: &Context<'_>) -> Result<Vec<OrderReturn>> {
        let db = ctx.data::<Database>().unwrap();
        let owner_id = ctx.data::<OwnerID>().unwrap();

        let orders = order::Entity::find()
            .filter(order::Column::RequesterId.eq(*owner_id))
            .all(db)
            .await?;

        Ok(orders.into_iter().map(Into::into).collect())
    }

    pub async fn orders_by_status(
        &self,
        ctx: &Context<'_>,
        status: OrderStatus,
    ) -> Result<Vec<OrderReturn>> {
        let db = ctx.data::<Database>().unwrap();

        let orders = order::Entity::find()
            .filter(order::Column::Status.eq(status))
            .all(db)
            .await?;

        Ok(orders.into_iter().map(Into::into).collect())
    }
}
