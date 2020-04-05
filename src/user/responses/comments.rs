use crate::responses::BasicListing;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CommentsData {
    pub body: String,
    pub body_html: String,
    pub link_title: String,
    pub link_url: String,
    pub subreddit: String,
    pub created: f64,
}

pub type Comments = BasicListing<CommentsData>;
