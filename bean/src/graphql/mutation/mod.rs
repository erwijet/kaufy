pub mod drink;
pub mod order;

use self::drink::DrinkMutation;
use self::order::OrderMutation;

#[derive(entity::async_graphql::MergedObject, Default)]
pub struct Mutation(DrinkMutation, OrderMutation);
