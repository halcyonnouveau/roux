//! # Subreddit Comment Responses
use serde::{Deserialize, Serialize};

use crate::models::{reply::MaybeReplies, response::BasicListing};

/// SubredditCommentsData, kind t3
/// Option for diff of `latest_comments` and `article_comments`,
/// see here: https://www.diffchecker.com/6qdlZr5a/
#[derive(Serialize, Debug, Deserialize)]
pub struct CommentData {
    /// Total awards
    pub total_awards_received: i32,
    /// Approved at (UTC)
    pub approved_at_utc: f64,
    /// Link id
    pub link_id: String,
    /// What is this
    pub author_flair_template_id: String,
    /// Likes, can be null in json --> Option
    pub likes: Option<bool>,
    /// Saved
    pub saved: bool,
    /// ID, every basic thing has an id
    pub id: String,
    /// Gilded
    pub gilded: i32,
    /// Archived
    pub archived: bool,
    /// No follow
    pub no_follow: bool,
    /// Auuthor
    pub author: String,
    /// Can mod post
    pub can_mod_post: bool,
    /// Created (UTC)
    pub created_utc: f64,
    /// Send replies
    pub send_replies: bool,
    /// Parent ID, full name of parent (so f.e t3_1fj3fj)
    pub parent_id: String,
    /// Score
    pub score: i32,
    /// Author fullname
    pub author_fullname: String,
    /// Over 18, only for latest comments (url/comments)
    pub over_18: Option<bool>,
    /// Approved by
    pub approved_by: String,
    /// Subreddit ID f.e "t5_2qh33"
    pub subreddit_id: Option<String>,
    /// Body
    pub body: String,
    /// Link title, only for latest comments (url/comments)
    pub link_title: Option<String>,
    /// Name, type (in this case t1 appended with "_" and then the id,so for example t1_lc6y7am)
    pub name: String,
    /// Patreon flair
    pub author_patreon_flair: bool,
    /// Downs?
    pub downs: i32,
    /// Is submitter
    pub is_submitter: bool,
    /// HTML
    pub body_html: String,
    /// Distinguished, can be null in json --> Option
    pub distinguished: Option<String>,
    /// if the comment was eddited
    pub edited: bool,
    /// Stickied
    pub stickied: bool,
    /// Premium
    pub author_premium: bool,
    /// Can guild
    pub can_gild: bool,
    /// Subreddit, name of subreddit (f.e. "funny")
    pub subreddit: String,
    /// Flair color
    pub author_flair_text_color: String,
    /// Score hidden
    pub score_hidden: bool,
    /// Permalink, url ending, used to request the comment, so f.e "/r/funny/comments/1dxyka2/let_me_do_it/lc6yg5s/"
    pub permalink: String,
    /// Number of reports, can be null in json --> Option
    pub num_reports: Option<i32>,
    /// only for latest comments (url/comments)
    pub num_comments: Option<i32>,
    /// depth of comment, only available in get_article_comments (url/comments/article_id)
    pub depth: Option<i32>,
    /// Permalink, only for latest comments (url/comments)
    pub link_permalink: Option<String>,
    /// Only for latest comments (url/comments and not url/comments/article_id)
    pub link_author: Option<String>,
    /// Sub name, prefixed with "r/" (f.e. "r/funny")
    pub subreddit_name_prefixed: String,
    /// Author flair
    pub author_flair_text: String,
    /// Link url, only for latest comments (url/comments)
    pub link_url: Option<String>,
    /// Created
    pub created: f64,
    /// Collapsed
    pub collapsed: bool,
    /// Controversiality
    pub controversiality: i32,
    /// Locked
    pub locked: bool,
    /// Quarantine, only for latest comments (url/comments)
    pub quarantine: Option<bool>,
    /// Subreddit type
    pub subreddit_type: String,
    /// UPS?
    pub ups: i32,
    /// Replies
    pub replies: Option<MaybeReplies>,
}

/// SubredditComments
pub type Comments = BasicListing<CommentData>;
