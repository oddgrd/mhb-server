use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Boulder database model
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Boulder {
    pub id: String,
    // TODO: title should be unique per Board and validated for length (and profanity?)
    pub title: String,
    /// The boulder's difficulty grade. The grade is first suggested by the boulder creator,
    /// and after the boulder is ascended it will be set by the average suggested grade
    /// of all ascentionists.
    // TODO: create a table holding the ascentionists grade suggestion, their ID and the
    // boulder ID, to more efficiently determine the grade?
    pub grade: i32,
    /// Whether the boulder is published. After a user creates a boulder, they will
    /// be able to modify it as they please before publishing. After publishing it will
    /// be public in the Board and the hold-coordinates will be immutable.
    pub published: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
