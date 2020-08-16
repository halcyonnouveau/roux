//! # Subreddit Moderator Responses
use crate::responses::{BasicThing, Listing};
use serde::Deserialize;

/// ModeratorsData
#[derive(Debug, Deserialize)]
pub struct ModeratorsData {
    /// The ID of the moderator
    pub id: String,
    /// The name of the moderator
    pub name: String,
    /// Author flair text
    pub author_flair_text: Option<String>,
}

/// Moderators
pub type Moderators = BasicThing<Listing<ModeratorsData>>;
