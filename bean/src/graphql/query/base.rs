use crate::db::Database;
use async_graphql::{Context, Object, Result};
use entity::{async_graphql, base, sea_orm::EntityTrait};

#[derive(Default)]
pub struct BaseQuery;

#[Object]
impl BaseQuery {
    async fn bases(&self, ctx: &Context<'_>) -> Result<Vec<base::Model>> {
        let db = ctx.data::<Database>().unwrap();
        Ok(base::Entity::find().all(db).await?)
    }
}
