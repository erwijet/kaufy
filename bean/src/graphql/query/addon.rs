use crate::db::Database;
use async_graphql::{Context, Object, Result};
use entity::{addon, async_graphql, sea_orm::EntityTrait};

#[derive(Default)]
pub struct AddonQuery;

#[Object]
impl AddonQuery {
    async fn addons(&self, ctx: &Context<'_>) -> Result<Vec<addon::Model>> {
        let db = ctx.data::<Database>().unwrap();
        Ok(addon::Entity::find().all(db).await?)
    }
}
