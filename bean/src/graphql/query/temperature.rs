use crate::db::Database;
use async_graphql::{Context, Object, Result};
use entity::{async_graphql, sea_orm::EntityTrait, temperature};

#[derive(Default)]
pub struct TemperatureQuery;

#[Object]
impl TemperatureQuery {
    async fn temperatures(&self, ctx: &Context<'_>) -> Result<Vec<temperature::Model>> {
        let db = ctx.data::<Database>().unwrap();
        Ok(temperature::Entity::find().all(db).await?)
    }
}
