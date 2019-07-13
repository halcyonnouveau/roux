use crate::util::defaults::default_string;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OverviewChildrenData {
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

#[derive(Debug, Deserialize)]
pub struct OverviewChildren {
    kind: String,
    data: OverviewChildrenData,
}

#[derive(Debug, Deserialize)]
pub struct OverviewData {
    children: Vec<OverviewChildren>,
}

#[derive(Debug, Deserialize)]
pub struct Overview {
    kind: String,
    data: OverviewData,
}
