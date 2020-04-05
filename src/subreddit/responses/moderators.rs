use crate::responses::{BasicThing, Listing};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ModeratorsData {
    /// The ID of the moderator
    pub id: String,
    /// The name of the moderator
    pub name: String,
    pub author_flair_text: Option<String>,
}

pub type Moderators = BasicThing<Listing<ModeratorsData>>;
