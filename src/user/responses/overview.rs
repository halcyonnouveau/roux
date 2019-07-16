use serde::Deserialize;
use crate::util::defaults::default_string;
use crate::responses::BasicListing;

#[derive(Debug, Deserialize)]
pub struct OverviewData {
    author: String,
    likes: Option<i32>,
    score: i32,
    subreddit: String,
    created: f64,
    #[serde(default = "default_string")]
    body: String,
    #[serde(default = "default_string")]
    link_title: String,
    #[serde(default = "default_string")]
    link_url: String,
}

pub type Overview  = BasicListing<OverviewData>;
