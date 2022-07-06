//! # Subreddit Moderator Responses 
use serde::Deserialize;

use crate::models::response::BasicListing;

/// ModeratorsData
#[derive(Debug, Deserialize)]
pub struct ModeratorData {
    /// The ID of the moderator
    pub id: String,
    /// The name of the moderator
    pub name: String,
    /// Author flair text
    pub author_flair_text: Option<String>,
}

/// Moderators
pub type Moderators = BasicListing<ModeratorData>;
