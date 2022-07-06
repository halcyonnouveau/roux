//! # User Overview Responses
use crate::models::{response::BasicListing};
use crate::util::defaults::default_string;
use serde::Deserialize;

/// OverviewData
#[derive(Debug, Deserialize)]
pub struct OverviewData {
    /// Author
    pub author: String,
    /// Likes
    pub likes: Option<i32>,
    /// Score
    pub score: i32,
    /// Subreddit
    pub subreddit: String,
    /// Created
    pub created: f64,
    /// Body
    #[serde(default = "default_string")]
    pub body: String,
    /// Link title
    #[serde(default = "default_string")]
    pub link_title: String,
    /// Link url
    #[serde(default = "default_string")]
    pub link_url: String,
}

/// Overview
pub type Overview = BasicListing<OverviewData>;
