pub mod addon;
pub mod base;
pub mod drink;
pub mod me;
pub mod order;
pub mod temperature;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(
    base::BaseQuery,
    addon::AddonQuery,
    temperature::TemperatureQuery,
    drink::DrinkQuery,
    me::MeQuery,
    order::OrderQuery
);
