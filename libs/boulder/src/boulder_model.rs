#![allow(missing_docs)]

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// The Boulder GraphQL and Database Model
#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize, SimpleObject)]
#[graphql(name = "Boulder")]
#[sea_orm(table_name = "boulders")]
pub struct Model {
    /// The Boulder's id.
    #[sea_orm(primary_key, column_type = "Text")]
    pub id: String,

    /// The Boulder's title.
    // TODO: title should be unique per Board and validated for length (and profanity?)
    pub title: String,

    /// The Boulder's difficulty grade. The grade is first suggested by the boulder creator,
    /// and after the boulder is ascended it will be set by the average suggested grade
    /// of all ascentionists.
    // TODO: create a table holding the ascentionists grade suggestion, their ID and the
    // boulder ID, to more efficiently determine the grade?
    pub grade: i32,

    /// Whether the Boulder is published. After a user creates a boulder, they will
    /// be able to modify it as they please before publishing. After publishing it will
    /// be public in the Board and the hold-coordinates will be immutable.
    pub published: bool,

    /// The date and time the Boulder was last updated.
    pub updated_at: DateTime,

    /// The date and time the Boulder was created.
    pub created_at: DateTime,
}

/// The Boulder GraphQL type is the same as the Boulder database Model.
pub type Boulder = Model;

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// A wrapper around `Option<Boulder>` to enable the trait implementations below.
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
