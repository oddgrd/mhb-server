use async_trait::async_trait;
use sea_orm::{entity::ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::sync::Arc;

use crate::boulder_model::{self, Boulder, BoulderOption};

#[async_trait]
pub trait BoulderServiceTrait: Sync + Send {
    /// Get an individual `Boulder` by id
    async fn get(&self, id: &str) -> anyhow::Result<Option<Boulder>>;

    /// Create a `Boulder` with the given title
    async fn create(&self, title: &str) -> anyhow::Result<Boulder>;
}

/// The default `BoulderService` struct.
pub struct BoulderService {
    /// The SeaOrm database connection, wrapped in an arc to allow using the same
    /// pool across threads
    db: Arc<DatabaseConnection>,
}

/// The default `BoulderService` implementation
impl BoulderService {
    /// Create a new `BoulderService` instance
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl BoulderServiceTrait for BoulderService {
    async fn get(&self, id: &str) -> anyhow::Result<Option<Boulder>> {
        let query = boulder_model::Entity::find_by_id(id.to_owned());

        let boulder = query.one(&*self.db).await?;

        let boulder: BoulderOption = boulder.into();

        Ok(boulder.into())
    }

    async fn create(&self, title: &str) -> anyhow::Result<Boulder> {
        let boulder = boulder_model::ActiveModel {
            title: Set(title.into()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        Ok(boulder)
    }
}
