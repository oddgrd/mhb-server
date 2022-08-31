use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};
use boulder::boulder_resolvers::BoulderQuery;
use std::sync::Arc;

use crate::graphql::Context;

#[derive(MergedObject, Default)]
pub struct Query(BoulderQuery);

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Create the graphql schema with dependencies injected into the context
pub fn create_schema(context: Arc<Context>) -> anyhow::Result<AppSchema> {
    Ok(
        Schema::build(Query::default(), EmptyMutation, EmptySubscription)
            .data(context.configuration.clone())
            .data(context.boulders.clone())
            .finish(),
    )
}
