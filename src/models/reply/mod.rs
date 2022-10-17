//! # Subreddit Comment Responses
use serde::{Deserialize, Serialize};

use crate::models::comment::CommentData;
use crate::models::response::BasicListing;

/// Doc
pub type Replies = BasicListing<CommentData>;

/// Replies can be more comments or an empty string
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeReplies {
    /// Reply
    Reply(Replies),
    /// String
    Str(String),
}
