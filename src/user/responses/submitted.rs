//! # User Submitted Responses
use crate::responses::BasicListing;
use serde::Deserialize;

/// Submitted
#[derive(Debug, Deserialize)]
pub struct SubmittedData {
    /// Subreddit
    pub subreddit: String,
    /// Title
    pub title: String,
    /// Selftext
    pub selftext: String,
    /// Thumbnail
    pub thumbnail: String,
    /// Score
    pub score: i32,
    /// Created
    pub created: f64,
    /// Domain
    pub domain: String,
    /// Is self
    pub is_self: bool,
}

/// Submitted
pub type Submitted = BasicListing<SubmittedData>;
