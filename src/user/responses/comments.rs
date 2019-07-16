use serde::Deserialize;
use crate::responses::BasicListing;

#[derive(Debug, Deserialize)]
pub struct CommentsData {
    body: String,
    body_html: String,
    link_title: String,
    link_url: String,
    subreddit: String,
    created: f64,
}

pub type Comments = BasicListing<CommentsData>;
