use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::{boulder_model::Boulder, boulder_service::BoulderServiceTrait};

/// The Query segment for Boulders
#[derive(Default)]
pub struct BoulderQuery {}

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
