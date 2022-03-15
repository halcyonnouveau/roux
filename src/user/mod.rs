//! # User
//! A read-only module to read data from for a specific user.
//!
//! # Usage
//! ```rust
//! use roux::User;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() {
//!     let user = User::new("beanpup_py");
//!     // Now you are able to:
//!
//!     // Get overview
//!     let overview = user.overview().await;
//!
//!     // Get submitted posts.
//!     let submitted = user.submitted().await;
//!
//!     // Get comments.
//!     let comments = user.comments().await;
//! }
//! ```

extern crate reqwest;
extern crate serde_json;

use crate::util::RouxError;
use reqwest::Client;

pub mod responses;
use crate::subreddit::responses::{Submissions, SubredditComments};
use responses::Overview;

/// User.
pub struct User {
    /// User's name.
    pub user: String,
    client: Client,
}

impl User {
    /// Create a new `User` instance.
    pub fn new(user: &str) -> User {
        User {
            user: user.to_owned(),
            client: Client::new(),
        }
    }

    /// Get user's overview.
    pub async fn overview(&self) -> Result<Vec<Overview>, RouxError> {
        let url = |user: &String, limit: u8, after: &String| {
            format!(
                "https://www.reddit.com/user/{}/overview/.json?sort=new&t=all&limit={}&after={}",
                user, limit, after
            )
        };
        let mut after = String::from("");
        let mut overviews: Vec<Overview> = vec![];

        loop {
            let over = self
                .client
                .get(&url(&self.user, 100, &after))
                .send()
                .await?
                .json::<Overview>()
                .await?;

            match &over.data.after {
                Some(a) => {
                    after = a.to_string();
                    overviews.push(over);
                }
                None => {
                    overviews.push(over);
                    break;
                }
            }
        }
        Ok(overviews)
    }

    /// Get user's submitted posts.
    pub async fn submitted(&self) -> Result<Vec<Submissions>, RouxError> {
        let url = |user: &String, limit: u8, after: &String| {
            format!(
                "https://www.reddit.com/user/{}/submitted/.json?sort=new&t=all&limit={}&after={}",
                user, limit, after
            )
        };
        let mut after = String::from("");
        let mut submissions: Vec<Submissions> = vec![];

        loop {
            let subs = self
                .client
                .get(&url(&self.user, 100, &after))
                .send()
                .await?
                .json::<Submissions>()
                .await?;

            match &subs.data.after {
                Some(a) => {
                    after = a.to_string();
                    submissions.push(subs);
                }
                None => {
                    submissions.push(subs);
                    break;
                }
            }
        }
        Ok(submissions)
    }

    /// Get user's submitted comments.
    pub async fn comments(&self) -> Result<Vec<SubredditComments>, RouxError> {
        let url = |user: &String, limit: u8, after: &String| {
            format!(
                "https://www.reddit.com/user/{}/comments/.json?sort=new&t=all&limit={}&after={}",
                user, limit, after
            )
        };
        let mut after = String::from("");
        let mut comments: Vec<SubredditComments> = vec![];

        loop {
            let comms = self
                .client
                .get(&url(&self.user, 100, &after))
                .send()
                .await?
                .json::<SubredditComments>()
                .await?;

            match &comms.data.after {
                Some(a) => {
                    after = a.to_string();
                    comments.push(comms);
                }
                None => {
                    comments.push(comms);
                    break;
                }
            }
        }
        Ok(comments)
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use tokio;

    #[tokio::test]
    async fn test_no_auth() {
        let user = User::new("beneater");

        // Test overview
        let overview = user.overview().await;
        assert!(overview.is_ok());

        // Test submitted
        let submitted = user.submitted().await;
        assert!(submitted.is_ok());

        // Test comments
        let comments = user.comments().await;
        assert!(comments.is_ok());
    }
}
