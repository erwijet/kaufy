use std::ops::Deref;

use async_graphql::{ComplexObject, Context, Object, Result};
use entity::async_graphql::{self, SimpleObject};
use entity::sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use entity::{
    addon::{self},
    base::{self},
    temperature::{self},
    user::{self},
};
use entity::{drink, drink_addon};

use crate::db::Database;
use crate::graphql::schema::OwnerID;

#[derive(SimpleObject)]
#[graphql(complex, concrete(name = "Drink", params()))]
pub struct DrinkReturn {
    id: i32,
    #[graphql(skip)]
    model: drink::Model,
}

impl std::ops::Deref for DrinkReturn {
    type Target = drink::Model;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl From<drink::Model> for DrinkReturn {
    fn from(value: drink::Model) -> Self {
        DrinkReturn {
            id: value.id,
            model: value,
        }
    }
}

#[ComplexObject]
impl DrinkReturn {
    async fn name(&self) -> &String {
        &self.deref().name
    }

    async fn temperature(&self, ctx: &Context<'_>) -> temperature::Model {
        let db = ctx.data::<Database>().unwrap();
        temperature::Entity::find_by_id(self.temp_id)
            .one(db)
            .await
            .unwrap()
            .unwrap()
    }

    async fn base(&self, ctx: &Context<'_>) -> base::Model {
        let db = ctx.data::<Database>().unwrap();
        base::Entity::find_by_id(self.base_id)
            .one(db)
            .await
            .unwrap()
            .unwrap()
    }

    async fn owner(&self, ctx: &Context<'_>) -> user::Model {
        let db = ctx.data::<Database>().unwrap();
        user::Entity::find_by_id(self.owner_id)
            .one(db)
            .await
            .unwrap()
            .unwrap()
    }

    async fn addons(&self, ctx: &Context<'_>) -> Vec<addon::Model> {
        let db = ctx.data::<Database>().unwrap();

        self.model
            .find_linked(drink_addon::Entity)
            .all(db)
            .await
            .unwrap()
    }
}

#[derive(Default)]
pub struct DrinkQuery;

#[Object]
impl DrinkQuery {
    pub async fn drinks(&self, ctx: &Context<'_>) -> Result<Vec<DrinkReturn>> {
        let db = ctx.data::<Database>().unwrap();

        let drinks = drink::Entity::find().all(db).await?;
        Ok(drinks.into_iter().map(Into::into).collect())
    }

    pub async fn my_drinks(&self, ctx: &Context<'_>) -> Result<Vec<DrinkReturn>> {
        let db = ctx.data::<Database>().unwrap();
        let owner_id = ctx.data::<OwnerID>().unwrap();

        let drinks = drink::Entity::find()
            .filter(drink::Column::OwnerId.eq(*owner_id))
            .all(db)
            .await?;
        Ok(drinks.into_iter().map(Into::into).collect())
    }
}
