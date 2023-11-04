pub mod drink;
pub mod tag;

use self::drink::DrinkMutation;

#[derive(entity::async_graphql::MergedObject, Default)]
pub struct Mutation(DrinkMutation);
