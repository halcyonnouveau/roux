//! # User Comment Responses
use crate::responses::BasicListing;
use serde::Deserialize;

/// UserCommentsData
#[derive(Debug, Deserialize)]
pub struct UserCommentsData {
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

/// UserComments
pub type UserComments = BasicListing<UserCommentsData>;
