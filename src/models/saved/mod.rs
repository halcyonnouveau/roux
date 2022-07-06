//! # Me Responses
use crate::models::response::BasicListing;
use crate::models::comment::CommentData;
use crate::models::submission::SubmissionData;
use serde::Deserialize;

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