//! # Me Responses
use serde::Deserialize;
use crate::responses::BasicListing;
use crate::subreddit::responses::SubmissionsData;
use crate::subreddit::responses::comments::SubredditCommentsData;

/// MeData
#[derive(Debug, Deserialize)]
pub struct MeData {
    /// ID
    pub id: String,
    /// Is employee
    pub is_employee: bool,
    /// Verified
    pub verified: bool,
    /// Over 18
    pub over_18: bool,
    /// Has verified email
    pub has_verified_email: bool,
    /// Is suspended
    pub is_suspended: bool,
    /// Has mail
    pub has_mail: bool,
    /// Inbox count
    pub inbox_count: f64,
    /// Created
    pub created: f64,
    /// Created (UTC)
    pub created_utc: f64,
    /// In beta
    pub in_beta: bool,
    /// Comment karma
    pub comment_karma: i32,
    /// Link karma
    pub link_karma: i32,
    /// Is mod
    pub is_mod: bool,
    /// Is gold
    pub is_gold: bool,
    /// Icon img
    pub icon_img: String,
}

/// The response from an add friend request
#[derive(Debug, Deserialize)]
pub struct Friend {
    /// Was the friend request a success
    pub success: bool,
}

/// A saved item can be a comment or post
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SavedData{
    /// Post
    Submission(SubmissionsData),
    /// Comment
    Comment(SubredditCommentsData),
}

/// Saved listing
pub type Saved = BasicListing<SavedData>;
