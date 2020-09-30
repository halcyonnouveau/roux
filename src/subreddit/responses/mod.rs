//! # Subreddit Responses
pub mod moderators;
pub use moderators::{Moderators, ModeratorsData};

pub mod submissions;
pub use submissions::{Submissions, SubmissionsData};

pub mod comments;
pub use comments::{SubredditComments, SubredditCommentsData};

pub mod subreddits;
pub use subreddits::{SubredditData, SubredditsListing};
