use async_graphql::{InputObject, SimpleObject};

use crate::boulder_model::Boulder;

#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct CreateBoulderInput {
    /// The boulder problem's title
    pub title: String,
    /// The creator's suggested grade
    pub grade: i32,
}

#[derive(Clone, Eq, PartialEq, SimpleObject)]
pub struct MutateBoulderResult {
    pub boulder: Option<Boulder>,
}
