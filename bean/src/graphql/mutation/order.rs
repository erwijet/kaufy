use async_graphql::{ComplexObject, Context, Object, Result};
use chrono::format::format;
use chrono::{format, Days, Duration, Local, Utc};
use entity::acl::Roleset;
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::order_status::{OrderStatus, OrderStatusVariant};
use entity::sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter,
    Set,
};
use entity::{drink, drink_addon, order};
use reqwest::StatusCode;

use crate::db::Database;
use crate::graphql::query::drink::DrinkReturn;
use crate::graphql::query::order::OrderReturn;
use crate::graphql::schema::OwnerID;
use crate::pusher::{self, PusherChannel};

#[derive(InputObject)]
pub struct OrderInputObject {
    pub drink_id: i32,
    pub desired_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DeleteOrderResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct OrderMutation;

#[Object]
impl OrderMutation {
    pub async fn add_order(
        &self,
        ctx: &Context<'_>,
        input: OrderInputObject,
    ) -> Result<OrderReturn> {
        let db = ctx.data::<Database>().unwrap();
        let owner_id = ctx.data::<OwnerID>().unwrap().clone();

        let cur_time = Utc::now();

        let order = order::ActiveModel {
            drink_id: Set(input.drink_id),
            requested_at: Set(cur_time.timestamp_millis()),
            requested_for: Set(input
                .desired_time
                .unwrap_or((cur_time + Duration::hours(1)).timestamp_millis())),
            status: Set(OrderStatus::Queued),
            requester_id: Set(owner_id),
            ..Default::default()
        }
        .insert(db)
        .await?;

        let entity::user::Model {
            given_name,
            family_name,
            ..
        } = entity::user::Entity::find_by_id(order.requester_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "user {}",
                order.requester_id
            )))?;

        let entity::drink::Model { name, .. } = entity::drink::Entity::find_by_id(order.drink_id)
            .one(db)
            .await?
            .unwrap();

        let requester_name = format!("{} {}", given_name, family_name.chars().nth(0).unwrap());

        pusher::notify_user(
            owner_id,
            &format!("Order #{}", order.id),
            "Your order has been placed",
        )
        .await?;

        pusher::notify_channel(
            PusherChannel::Orders,
            "New Order",
            &*format!(
                "{requester_name}. just queued order #{} ({})",
                order.id, name
            ),
        )
        .await?;

        Ok(order.into())
    }

    pub async fn cancel_order(&self, ctx: &Context<'_>, id: i32) -> Result<OrderReturn> {
        let db = ctx.data::<Database>().unwrap();
        let actor_id = ctx.data::<OwnerID>().unwrap().clone();

        let order = entity::order::Entity::find_by_id(id)
            .one(db)
            .await?
            .unwrap();

        if let OrderStatus::Cancelled | OrderStatus::Done = order.status {
            Err(DbErr::Custom("Cannot cancel closed order".into()))?
        }

        let actor = entity::user::Entity::find_by_id(actor_id)
            .one(db)
            .await?
            .unwrap();

        if actor_id != order.requester_id
            && !Roleset::from(actor.roleset).contains(&entity::acl::AclRole::Barista)
        {
            Err(DbErr::Custom("Not allowed".into()))?
        }

        let entity::drink::Model { name, .. } = entity::drink::Entity::find_by_id(order.drink_id)
            .one(db)
            .await?
            .unwrap();

        let res = entity::order::ActiveModel {
            id: Set(order.id),
            status: Set(OrderStatus::Cancelled),
            ..Default::default()
        }
        .update(db)
        .await?;

        pusher::notify_user(
            res.requester_id,
            &format!("Order #{} | {}", res.id, name),
            "Your order has been cancelled",
        )
        .await?;

        Ok(res.into())
    }

    pub async fn update_order_status(
        &self,
        ctx: &Context<'_>,
        id: i32,
        new_status: OrderStatus,
    ) -> Result<OrderReturn> {
        let db = ctx.data::<Database>().unwrap();
        let actor_id = ctx.data::<OwnerID>().unwrap().clone();

        let order = entity::order::Entity::find_by_id(id)
            .one(db)
            .await?
            .unwrap();

        let owner = entity::user::Entity::find_by_id(actor_id)
            .one(db)
            .await?
            .unwrap();

        if !Roleset::from(owner.roleset).contains(&entity::acl::AclRole::Barista) {
            Err(DbErr::Custom("Not allowed".into()))?
        }

        Ok(entity::order::ActiveModel {
            id: Set(order.id),
            status: Set(new_status),
            ..Default::default()
        }
        .update(db)
        .await?
        .into())
    }
}
