use serde::Deserialize;
use crate::responses::BasicListing;

#[derive(Debug, Deserialize)]
pub struct SubmittedData {
    subreddit: String,
    title: String,
    thumbnail: String,
    score: i32,
    created: f64,
    domain: String,
    is_self: bool,
}

pub type Submitted = BasicListing<SubmittedData>;
