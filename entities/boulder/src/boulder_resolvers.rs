use std::sync::Arc;

use async_graphql::{Context, Object, Result};

use crate::{boulder_model::Boulder, boulder_service::BoulderService};

/// The Query segment for Boulders
#[derive(Default)]
pub struct BouldersQuery {}

#[Object]
impl BouldersQuery {
    /// Get a Boulder by id from the graphql context
    async fn get_boulder(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The Boulder id")] id: String,
    ) -> Result<Option<Boulder>> {
        let boulders = ctx.data_unchecked::<Arc<dyn BoulderService>>();

        Ok(boulders.get(&id).await?)
    }
}
