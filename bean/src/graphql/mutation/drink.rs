use async_graphql::{ComplexObject, Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set,
};
use entity::{drink, drink_addon};

use crate::db::Database;
use crate::graphql::query::drink::DrinkReturn;
use crate::graphql::schema::OwnerID;

#[derive(InputObject)]
pub struct DrinkInputObject {
    pub name: String,
    pub temperature_id: i32,
    pub base_id: i32,
    pub addon_ids: Vec<i32>,
}

#[derive(SimpleObject)]
pub struct DeleteDrinkResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct DrinkMutation;

#[Object]
impl DrinkMutation {
    pub async fn add_drink(
        &self,
        ctx: &Context<'_>,
        input: DrinkInputObject,
    ) -> Result<DrinkReturn> {
        let db = ctx.data::<Database>().unwrap();
        let owner_id = ctx.data::<OwnerID>().unwrap().clone();

        let drink = drink::ActiveModel {
            name: Set(input.name),
            base_id: Set(input.base_id),
            temp_id: Set(input.temperature_id),
            owner_id: Set(owner_id),
            ..Default::default()
        }
        .insert(db)
        .await?;

        for addon_id in input.addon_ids {
            drink_addon::ActiveModel {
                addon_id: Set(addon_id),
                drink_id: Set(drink.id),
            }
            .insert(db)
            .await?;
        }

        Ok(drink.into())
    }

    pub async fn update_drink(
        &self,
        ctx: &Context<'_>,
        id: i32,
        update: DrinkInputObject,
    ) -> Result<DrinkReturn> {
        let db = ctx.data::<Database>().unwrap();
        let owner_id = ctx.data::<OwnerID>().unwrap();

        let drink = drink::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Not found")?;

        if drink.owner_id != *owner_id {
            Err("No ownership")?;
        }

        drink_addon::Entity::delete_many()
            .filter(drink_addon::Column::DrinkId.eq(drink.id))
            .exec(db)
            .await?;

        for addon_id in update.addon_ids {
            drink_addon::ActiveModel {
                addon_id: Set(addon_id),
                drink_id: Set(drink.id),
            }
            .insert(db)
            .await?;
        }

        let mut active_drink: drink::ActiveModel = drink.into();

        active_drink.name = Set(update.name);
        active_drink.base_id = Set(update.base_id);
        active_drink.temp_id = Set(update.temperature_id);
        
        Ok(active_drink.update(db).await?.into())
    }

    pub async fn delete_drink(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteDrinkResult> {
        let db = ctx.data::<Database>().unwrap();
        let owner_id = ctx.data::<OwnerID>().unwrap();

        let drink = drink::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Not found")?;

        if drink.owner_id != *owner_id {
            Err("No ownership")?;
        }

        // drop all addons bound to this drink
        drink_addon::Entity::delete_many()
            .filter(drink_addon::Column::DrinkId.eq(drink.id))
            .exec(db)
            .await?;

        let res = drink::Entity::delete_by_id(drink.id).exec(db).await?;

        if res.rows_affected <= 1 {
            Ok(DeleteDrinkResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }
}