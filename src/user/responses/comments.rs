//! # User Comment Responses
use crate::responses::BasicListing;
use serde::Deserialize;

/// CommentsData
#[derive(Debug, Deserialize)]
pub struct CommentsData {
    /// Body
    pub body: String,
    /// Body HTML
    pub body_html: String,
    /// Link title
    pub link_title: String,
    /// Link url
    pub link_url: String,
    /// Subreddit
    pub subreddit: String,
    /// Created
    pub created: f64,
}

/// Comments
pub type Comments = BasicListing<CommentsData>;
