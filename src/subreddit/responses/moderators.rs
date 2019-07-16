use serde::Deserialize;
use crate::responses::{BasicThing, Listing};

#[derive(Debug, Deserialize)]
pub struct ModeratorsData {
    /// The ID of the moderator
    id: String,
    /// The name of the moderator
    name: String,
    author_flair_text: Option<String>,
}

pub type Moderators = BasicThing<Listing<ModeratorsData>>;
