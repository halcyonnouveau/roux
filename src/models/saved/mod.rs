//! # Me Responses

use serde::Deserialize;

use crate::models::comment::CommentData;
use crate::models::response::BasicListing;
use crate::models::submission::SubmissionData;

/// A saved item can be a comment or post
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SavedData {
    /// Post
    Submission(SubmissionData),
    /// Comment
    Comment(CommentData),
}

/// Saved listing
pub type Saved = BasicListing<SavedData>;
