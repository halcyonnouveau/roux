use crate::{responses::BasicListing, util::defaults::default_string};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OverviewData {
    pub author: String,
    pub likes: Option<i32>,
    pub score: i32,
    pub subreddit: String,
    pub created: f64,
    #[serde(default = "default_string")]
    pub body: String,
    #[serde(default = "default_string")]
    pub link_title: String,
    #[serde(default = "default_string")]
    pub link_url: String,
}

pub type Overview = BasicListing<OverviewData>;
