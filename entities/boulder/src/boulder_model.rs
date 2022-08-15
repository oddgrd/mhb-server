#![allow(missing_docs)]

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// The User GraphQL and Database Model
#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize, SimpleObject)]
#[graphql(name = "Boulder")]
#[sea_orm(table_name = "boulders")]
pub struct Model {
    /// The Boulder id
    #[sea_orm(primary_key, column_type = "Text")]
    pub id: String,

    /// The date the Boulder was created
    pub created_at: DateTime,

    /// The date the Boulder was last updated
    pub updated_at: DateTime,

    /// The Boulder's difficulty grade
    pub grade: i32,

    /// Whether the Boulder is active
    pub is_active: bool,
}

/// The Boulder GraphQL type is the same as the database Model
pub type Boulder = Model;

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// A wrapper around `Option<User>` to enable the trait implementations below
pub struct BoulderOption(pub Option<Boulder>);

impl From<Option<Model>> for BoulderOption {
    fn from(data: Option<Model>) -> BoulderOption {
        BoulderOption(data)
    }
}

impl From<BoulderOption> for Option<Boulder> {
    fn from(boulder: BoulderOption) -> Option<Boulder> {
        boulder.0
    }
}
