use crate::responses::BasicListing;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SubmittedData {
    pub subreddit: String,
    pub title: String,
    pub thumbnail: String,
    pub score: i32,
    pub created: f64,
    pub domain: String,
    pub is_self: bool,
}

pub type Submitted = BasicListing<SubmittedData>;
