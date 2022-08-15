use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::boulder_model::{self, Boulder, BoulderOption};

#[async_trait]
pub trait BoulderService: Sync + Send {
    /// Get an individual `Boulder` by id
    async fn get(&self, id: &str) -> anyhow::Result<Option<Boulder>>;
}

/// The default `BoulderService` struct.
pub struct DefaultBoulderService {
    /// The SeaOrm database connection, wrapped in an arc to allow using the same
    /// pool across threads
    db: Arc<DatabaseConnection>,
}

/// The default `BoulderService` implementation
impl DefaultBoulderService {
    /// Create a new `BoulderService` instance
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl BoulderService for DefaultBoulderService {
    async fn get(&self, id: &str) -> anyhow::Result<Option<Boulder>> {
        let query = boulder_model::Entity::find_by_id(id.to_owned());

        let boulder = query.one(&*self.db).await?;

        let boulder: BoulderOption = boulder.into();

        Ok(boulder.into())
    }
}
