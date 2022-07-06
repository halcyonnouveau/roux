//! # Subreddit Comment Responses
use crate::models::{response::BasicListing, reply::MaybeReplies};
use serde::Deserialize;

/// SubredditCommentsData
/// Everything is an option to deal with both `latest_comments` and `article_comments`
#[derive(Debug, Deserialize)]
pub struct CommentData {
    /// Total awards
    pub total_awards_received: Option<i32>,
    /// Approved at (UTC)
    pub approved_at_utc: Option<f64>,
    /// Link id
    pub link_id: Option<String>,
    /// What is this
    pub author_flair_template_id: Option<String>,
    /// Likes
    pub likes: Option<bool>,
    /// Saved
    pub saved: Option<bool>,
    /// ID
    pub id: Option<String>,
    /// Gilded
    pub gilded: Option<i32>,
    /// Archived
    pub archived: Option<bool>,
    /// No follow
    pub no_follow: Option<bool>,
    /// Auuthor
    pub author: Option<String>,
    /// Can mod post
    pub can_mod_post: Option<bool>,
    /// Created (UTC)
    pub created_utc: Option<f64>,
    /// Send replies
    pub send_replies: Option<bool>,
    /// Parent ID
    pub parent_id: Option<String>,
    /// Score
    pub score: Option<i32>,
    /// Author fullname
    pub author_fullname: Option<String>,
    /// Over 18
    pub over_18: Option<bool>,
    /// Approved by
    pub approved_by: Option<String>,
    /// Subreddit ID
    pub subreddit_id: Option<String>,
    /// Body
    pub body: Option<String>,
    /// Link title
    pub link_title: Option<String>,
    /// Name
    pub name: Option<String>,
    /// Patreon flair
    pub author_patreon_flair: Option<bool>,
    /// Downs?
    pub downs: Option<i32>,
    /// Is submitter
    pub is_submitter: Option<bool>,
    /// HTML
    pub body_html: Option<String>,
    /// Distinguished
    pub distinguished: Option<String>,
    /// Stickied
    pub stickied: Option<bool>,
    /// Premium
    pub author_premium: Option<bool>,
    /// Can guild
    pub can_gild: Option<bool>,
    /// Subreddit
    pub subreddit: Option<String>,
    /// Flair color
    pub author_flair_text_color: Option<String>,
    /// Score hidden
    pub score_hidden: Option<bool>,
    /// Permalink
    pub permalink: Option<String>,
    /// Number of reports
    pub num_reports: Option<i32>,
    /// Permalink
    pub link_permalink: Option<String>,
    /// Author link
    pub link_author: Option<String>,
    /// Sub name
    pub subreddit_name_prefixed: Option<String>,
    /// Author flair
    pub author_flair_text: Option<String>,
    /// Link url
    pub link_url: Option<String>,
    /// Created
    pub created: Option<f64>,
    /// Collapsed
    pub collapsed: Option<bool>,
    /// Controversiality
    pub controversiality: Option<i32>,
    /// Locked
    pub locked: Option<bool>,
    /// Quarantine
    pub quarantine: Option<bool>,
    /// Subreddit type
    pub subreddit_type: Option<String>,
    /// UPS?
    pub ups: Option<i32>,
    /// Replies
    pub replies: Option<MaybeReplies>,
}

/// SubredditComments
pub type Comments = BasicListing<CommentData>;
