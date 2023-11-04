pub mod addon;
pub mod base;
pub mod temperature;
pub mod drink;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(base::BaseQuery, addon::AddonQuery, temperature::TemperatureQuery, drink::DrinkQuery);
