use ::async_graphql::{EmptySubscription, Schema};

use crate::{
    db::Database,
    graphql::{mutation::Mutation, query::Query},
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub type OwnerID = i32;

/// Builds the gql schema, and attaches the DB context
pub async fn build_schema(owner_id: OwnerID) -> AppSchema {
    let db = Database::new().await;

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .data(owner_id)
        .finish()
}
