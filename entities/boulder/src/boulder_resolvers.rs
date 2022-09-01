use async_graphql::{Context, Object, Result};
use http::StatusCode;
use mhb_utils::errors::as_graphql_error;
use std::sync::Arc;

use crate::{
    boulder_model::Boulder,
    boulder_mutations::{CreateBoulderInput, MutateBoulderResult},
    boulder_service::BoulderServiceTrait,
};

/// The Query segment for Boulders
#[derive(Default)]
pub struct BoulderQuery {}

/// The Mutation segment for Boulders
#[derive(Default)]
pub struct BoulderMutation {}

#[Object]
impl BoulderQuery {
    /// Get a Boulder by id from the graphql context
    async fn get_boulder(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The Boulder id")] id: String,
    ) -> Result<Option<Boulder>> {
        let boulders = ctx.data_unchecked::<Arc<dyn BoulderServiceTrait>>();

        Ok(boulders.get(&id).await?)
    }
}

#[Object]
impl BoulderMutation {
    async fn create_boulder(
        &self,
        ctx: &Context<'_>,
        input: CreateBoulderInput,
    ) -> Result<MutateBoulderResult> {
        let boulders = ctx.data_unchecked::<Arc<dyn BoulderServiceTrait>>();

        let boulder = boulders
            .create(&input.title)
            .await
            .map_err(as_graphql_error(
                "Error while creating boulder",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(MutateBoulderResult {
            boulder: Some(boulder),
        })
    }
}
