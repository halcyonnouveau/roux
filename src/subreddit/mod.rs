//! # Subreddit
//! A read-only module to read data from a specific subreddit.
//!
//! # Basic Usage
//! ```rust,no_run
//! use roux::Subreddit;
//! let subreddit = Subreddit::new("rust");
//! // Now you are able to:
//! // Get moderators.
//! let moderators = subreddit.moderators();
//! // Get hot posts with limit = 25.
//! let hot = subreddit.hot(25, None);
//! // Get rising posts with limit = 30.
//! let rising = subreddit.rising(30, None);
//! // Get top posts with limit = 10.
//! let top = subreddit.top(10, None);
//! // Get latest comments.
//! // `depth` and `limit` are optional.
//! let latest_comments = subreddit.latest_comments(None, Some(25));
//! // Get comments from a submission.
//! let article_id = &hot.unwrap().data.children.first().unwrap().data.id.clone();
//! let article_comments = subreddit.article_comments(article_id, None, Some(25));
//! ```
//! # Usage with feed options
//! ```rust,no_run
//! use roux::Subreddit;
//! use roux::util::FeedOption;
//!
//! let subreddit = Subreddit::new("astolfo");
//!
//! // Gets top 10
//! let top = subreddit.top(10, None);
//!
//! // Get after param from `top`
//! let after = top.unwrap().data.after.unwrap();
//! let options = FeedOption::new().after(&after);
//!
//! // Gets next 25
//! let next_top = subreddit.top(25, Some(options));
//! ```

extern crate reqwest;
extern crate serde_json;

use crate::util::{FeedOption, RouxError};
use reqwest::Client;

mod responses;
use responses::{Comments, Moderators, Submissions};

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
        Ok(self
            .client
            .get(&format!("{}/about/moderators/.json", self.url))
            .send()?
            .json::<Moderators>()?)
    }

    fn get_feed(
        &self,
        ty: &str,
        limit: u32,
        options: Option<FeedOption>,
    ) -> Result<Submissions, RouxError> {
        let url = &mut format!("{}/{}.json?limit={}", self.url, ty, limit);

        if !options.is_none() {
            let option = options.unwrap();

            if !option.after.is_none() {
                url.push_str(&mut format!("&after={}", option.after.unwrap().to_owned()));
            } else if !option.before.is_none() {
                url.push_str(&mut format!(
                    "&before={}",
                    option.before.unwrap().to_owned()
                ));
            }

            if !option.count.is_none() {
                url.push_str(&mut format!("&count={}", option.count.unwrap()));
            }
        }

        Ok(self
            .client
            .get(&url.to_owned())
            .send()?
            .json::<Submissions>()?)
    }

    fn get_comment_feed(
        &self,
        ty: &str,
        depth: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Comments, RouxError> {
        let url = &mut format!("{}/{}.json?", self.url, ty);

        if !depth.is_none() {
            url.push_str(&mut format!("&depth={}", depth.unwrap()));
        }

        if !limit.is_none() {
            url.push_str(&mut format!("&limit={}", limit.unwrap()));
        }

        // This is one of the dumbest APIs I've ever seen.
        // The comments for a subreddit are stored in a normal hash map
        // but for posts the comments are in an array with the ONLY item
        // being same hash map as the one for subreddits...
        if url.contains("comments/") {
            let mut comments = self
                .client
                .get(&url.to_owned())
                .send()?
                .json::<Vec<Comments>>()?;

            Ok(comments.pop().unwrap())
        } else {
            Ok(self
                .client
                .get(&url.to_owned())
                .send()?
                .json::<Comments>()?)
        }
    }

    /// Get hot posts.
    pub fn hot(&self, limit: u32, options: Option<FeedOption>) -> Result<Submissions, RouxError> {
        self.get_feed("hot", limit, options)
    }

    /// Get rising posts.
    pub fn rising(
        &self,
        limit: u32,
        options: Option<FeedOption>,
    ) -> Result<Submissions, RouxError> {
        self.get_feed("rising", limit, options)
    }

    /// Get top posts.
    pub fn top(&self, limit: u32, options: Option<FeedOption>) -> Result<Submissions, RouxError> {
        // TODO: time filter
        self.get_feed("top", limit, options)
    }

    /// Get latest posts.
    pub fn latest(
        &self,
        limit: u32,
        options: Option<FeedOption>,
    ) -> Result<Submissions, RouxError> {
        self.get_feed("new", limit, options)
    }

    /// Get latest comments.
    pub fn latest_comments(
        &self,
        depth: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Comments, RouxError> {
        self.get_comment_feed("comments", depth, limit)
    }

    /// Get comments from article.
    pub fn article_comments(
        &self,
        article: &str,
        depth: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Comments, RouxError> {
        self.get_comment_feed(&format!("comments/{}", article), depth, limit)
    }
}

#[cfg(test)]
mod tests {
    use super::FeedOption;
    use super::Subreddit;

    #[test]
    fn test_no_auth() {
        let subreddit = Subreddit::new("rust");

        // Test moderators
        let moderators = subreddit.moderators();
        assert!(moderators.is_ok());

        // Test feeds
        let hot = subreddit.hot(25, None);
        assert!(hot.is_ok());

        let rising = subreddit.rising(25, None);
        assert!(rising.is_ok());

        let top = subreddit.top(25, None);
        assert!(top.is_ok());

        let latest_comments = subreddit.latest_comments(None, Some(25));
        assert!(latest_comments.is_ok());

        let article_id = &hot.unwrap().data.children.first().unwrap().data.id.clone();
        let article_comments = subreddit.article_comments(article_id, None, Some(25));
        assert!(article_comments.is_ok());

        // Test with feed options
        let after = top.unwrap().data.after.unwrap();
        let options = FeedOption::new().after(&after);

        let next_top = subreddit.top(25, Some(options));
        assert!(next_top.is_ok());
    }
}
