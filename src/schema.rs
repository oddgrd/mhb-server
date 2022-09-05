use async_graphql::{EmptySubscription, MergedObject, Schema};
use boulder::boulder_resolvers::{BoulderMutation, BoulderQuery};
use std::sync::Arc;

use crate::graphql::Context;

#[derive(MergedObject, Default)]
pub struct Query(BoulderQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(BoulderMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Create the graphql schema with dependencies injected into the context
pub fn create_schema(context: Arc<Context>) -> anyhow::Result<AppSchema> {
    Ok(
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(context.boulders.clone())
            .finish(),
    )
}
