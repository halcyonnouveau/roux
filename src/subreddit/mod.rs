//! A read-only `Subreddit` class.
//! # Usage
//! ```rust,ignore
//! use roux::Subreddit;
//! let subreddit = Subreddit::new("subreddt_name");
//! ```
//! # Get Moderators
//! ```rust,ignore
//! let moderators = subreddit.moderators();
//! ```
//! # Get Hot Posts
//! ```rust,ignore
//! let hot = subreddit.hot();
//! ```
//! # Get Rising Posts
//! ```rust,ignore
//! let rising = subreddit.rising();
//! ```
//! # Get Top Posts
//! ```rust,ignore
//! let top = subreddit.top();
//! ```

extern crate reqwest;
extern crate serde_json;

use reqwest::Client;
use crate::util::RouxError;

mod responses;
use responses::{Moderators, Submissions};

/// Subreddit.
pub struct Subreddit {
    /// Name of subreddit.
    pub name: String,
    url: String,
    client: Client,
}

impl Subreddit {
    /// Create a new `Subreddit` instance.
    pub fn new(name: &str) -> Subreddit {
        let subreddit_url = format!("https://www.reddit.com/r/{}", name);

        Subreddit {
            name: name.to_owned(),
            url: subreddit_url,
            client: Client::new(),
        }
    }

    /// Get moderators.
    pub fn moderators(&self) -> Result<Moderators, RouxError> {
        Ok(self.client
            .get(&format!("{}/about/moderators/.json", self.url))
            .send()?
            .json::<Moderators>()?)
    }

    fn get_feed(&self, ty: &str, limit: u32) -> Result<Submissions, RouxError> {
        Ok(self.client
            .get(&format!("{}/{}.json?limit={}", self.url, ty, limit))
            .send()?
            .json::<Submissions>()?)
    }

    /// Get hot posts.
    pub fn hot(&self, limit: u32) -> Result<Submissions, RouxError> {
        self.get_feed("hot", limit)
    }

    /// Get rising posts.
    pub fn rising(&self, limit: u32) -> Result<Submissions, RouxError> {
        self.get_feed("rising", limit)
    }

    /// Get top posts.
    pub fn top(&self, limit: u32) -> Result<Submissions, RouxError> {
        // TODO: time filter
        self.get_feed("top", limit)
    }
}

#[cfg(test)]
mod tests {
    use super::Subreddit;

    #[test]
    fn test_no_auth() {
        let subreddit = Subreddit::new("rust");

        // Test moderators
        let moderators = subreddit.moderators();
        assert!(moderators.is_ok());

        // Test feeds
        let hot = subreddit.hot(25);
        assert!(hot.is_ok());
        let rising = subreddit.rising(25);
        assert!(rising.is_ok());
        let top = subreddit.top(25);
        assert!(top.is_ok());
    }
}
