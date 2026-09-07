//! # Subreddit
//! A read-only module to read data from a specific subreddit.
//!
//! Reddit no longer serves these endpoints to unauthenticated clients, so a
//! `Subreddit` should be created through
//! [`Reddit::subreddit`](crate::Reddit::subreddit), which authenticates with
//! an application-only token when no username and password are set.
//!
//! # Basic Usage
//! ```no_run
//! use roux::Reddit;
//! # #[cfg(not(feature = "blocking"))]
//! # use tokio;
//!
//! # #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! # #[maybe_async::maybe_async]
//! # async fn main() {
//! let subreddit = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .subreddit("rust")
//!     .await
//!     .unwrap();
//! // Now you are able to:
//!
//! // Get moderators.
//! let moderators = subreddit.moderators().await;
//!
//! // Get hot posts with limit = 25.
//! let hot = subreddit.hot(25, None).await;
//!
//! // Get rising posts with limit = 30.
//! let rising = subreddit.rising(30, None).await;
//!
//! // Get top posts with limit = 10.
//! let top = subreddit.top(10, None).await;
//!
//! // Get latest comments.
//! // `depth` and `limit` are optional.
//! let latest_comments = subreddit.latest_comments(None, Some(25)).await;
//!
//! // Get comments from a submission.
//! let article_id = &hot.unwrap().data.children.first().unwrap().data.id.clone();
//! let article_comments = subreddit.article_comments(article_id, None, Some(25));
//! # }
//! ```
//!
//! # Usage with feed options
//!
//! ```no_run
//! use roux::Reddit;
//! use roux::util::{FeedOption, TimePeriod};
//! # #[cfg(not(feature = "blocking"))]
//! # use tokio;
//!
//! # #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! # #[maybe_async::maybe_async]
//! # async fn main() {
//! let subreddit = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .subreddit("astolfo")
//!     .await
//!     .unwrap();
//!
//! // Gets top 10 posts from this month
//! let options = FeedOption::new().period(TimePeriod::ThisMonth);
//! let top = subreddit.top(25, Some(options)).await;
//!
//! // Gets hot 10
//! let hot = subreddit.hot(25, None).await;
//!
//! // Get after param from `hot`
//! let after = hot.unwrap().data.after.unwrap();
//! let after_options = FeedOption::new().after(&after);
//!
//! // Gets next 25
//! let next_hot = subreddit.hot(25, Some(after_options)).await;
//! # }
//! ```
pub mod response;
extern crate serde_json;

use crate::models::subreddit::response::{SubredditData, SubredditResponse, SubredditsData};

use crate::client::Client;
use crate::util::defaults::default_client;
use crate::util::{FeedOption, RouxError};

use crate::models::{Comments, Moderators, Submissions};

/// Access subreddits API
pub struct Subreddits;

impl Subreddits {
    /// Search subreddits without authentication.
    ///
    /// Reddit blocks unauthenticated requests to this endpoint, so this will
    /// almost certainly fail with a 403. Use
    /// [`Reddit::search_subreddits`](crate::Reddit::search_subreddits) or
    /// [`Subreddits::search_oauth`] instead.
    #[deprecated(
        since = "2.3.0",
        note = "Reddit blocks unauthenticated API access; use `Reddit::search_subreddits` instead"
    )]
    #[maybe_async::maybe_async]
    pub async fn search(
        name: &str,
        limit: Option<u32>,
        options: Option<FeedOption>,
    ) -> Result<SubredditsData, RouxError> {
        let url = format!("https://www.reddit.com/subreddits/search.json?q={}", name);
        Self::search_with(url, limit, options, &default_client()).await
    }

    /// Search subreddits using an oauth client from the `Reddit` module.
    #[maybe_async::maybe_async]
    pub async fn search_oauth(
        name: &str,
        limit: Option<u32>,
        options: Option<FeedOption>,
        client: &Client,
    ) -> Result<SubredditsData, RouxError> {
        let url = format!("https://oauth.reddit.com/subreddits/search.json?q={}", name);
        Self::search_with(url, limit, options, client).await
    }

    #[maybe_async::maybe_async]
    async fn search_with(
        mut url: String,
        limit: Option<u32>,
        options: Option<FeedOption>,
        client: &Client,
    ) -> Result<SubredditsData, RouxError> {
        let url = &mut url;

        if let Some(limit) = limit {
            url.push_str(&format!("&limit={}", limit));
        }

        if let Some(options) = options {
            options.build_url(url);
        }

        Ok(client
            .get(&url.to_owned())
            .send()
            .await?
            .json::<SubredditsData>()
            .await?)
    }
}

/// Subreddit
pub struct Subreddit {
    /// Name of subreddit.
    pub name: String,
    url: String,
    client: Client,
    is_oauth: bool,
}

impl Subreddit {
    /// Create a new unauthenticated `Subreddit` instance.
    ///
    /// Reddit blocks unauthenticated requests to these endpoints, so this
    /// will almost certainly fail with a 403. Use
    /// [`Reddit::subreddit`](crate::Reddit::subreddit) instead.
    #[deprecated(
        since = "2.3.0",
        note = "Reddit blocks unauthenticated API access; use `Reddit::subreddit` instead"
    )]
    pub fn new(name: &str) -> Subreddit {
        let subreddit_url = format!("https://www.reddit.com/r/{}", name);

        Subreddit {
            name: name.to_owned(),
            url: subreddit_url,
            client: default_client(),
            is_oauth: false,
        }
    }

    /// Create a new authenticated `Subreddit` instance using an oauth client
    /// from the `Reddit` module.
    pub fn new_oauth(name: &str, client: &Client) -> Subreddit {
        let subreddit_url = format!("https://oauth.reddit.com/r/{}", name);

        Subreddit {
            name: name.to_owned(),
            url: subreddit_url,
            client: client.to_owned(),
            is_oauth: true,
        }
    }

    /// Get moderators (requires authentication)
    #[maybe_async::maybe_async]
    pub async fn moderators(&self) -> Result<Moderators, RouxError> {
        if self.is_oauth {
            Ok(self
                .client
                .get(&format!("{}/about/moderators/.json", self.url))
                .send()
                .await?
                .json::<Moderators>()
                .await?)
        } else {
            Err(RouxError::OAuthClientRequired)
        }
    }

    /// Get subreddit data.
    #[maybe_async::maybe_async]
    pub async fn about(&self) -> Result<SubredditData, RouxError> {
        Ok(self
            .client
            .get(&format!("{}/about/.json", self.url))
            .send()
            .await?
            .json::<SubredditResponse>()
            .await?
            .data)
    }

    #[maybe_async::maybe_async]
    async fn get_feed(
        &self,
        ty: &str,
        limit: u32,
        options: Option<FeedOption>,
    ) -> Result<Submissions, RouxError> {
        let url = &mut format!("{}/{}.json?limit={}", self.url, ty, limit);

        if let Some(options) = options {
            options.build_url(url);
        }

        Ok(self
            .client
            .get(&url.to_owned())
            .send()
            .await?
            .json::<Submissions>()
            .await?)
    }

    #[maybe_async::maybe_async]
    async fn get_comment_feed(
        &self,
        ty: &str,
        depth: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Comments, RouxError> {
        let url = &mut format!("{}/{}.json?", self.url, ty);

        if let Some(depth) = depth {
            url.push_str(&format!("&depth={}", depth));
        }

        if let Some(limit) = limit {
            url.push_str(&format!("&limit={}", limit));
        }

        // This is one of the dumbest APIs I've ever seen.
        // The comments for a subreddit are stored in a normal hash map
        // but for posts the comments are in an array with the ONLY item
        // being same hash map as the one for subreddits...
        if url.contains("comments/") {
            let mut comments = self
                .client
                .get(&url.to_owned())
                .send()
                .await?
                .json::<Vec<Comments>>()
                .await?;

            Ok(comments.pop().unwrap())
        } else {
            Ok(self
                .client
                .get(&url.to_owned())
                .send()
                .await?
                .json::<Comments>()
                .await?)
        }
    }

    /// Get hot posts.
    #[maybe_async::maybe_async]
    pub async fn hot(
        &self,
        limit: u32,
        options: Option<FeedOption>,
    ) -> Result<Submissions, RouxError> {
        self.get_feed("hot", limit, options).await
    }

    /// Get rising posts.
    #[maybe_async::maybe_async]
    pub async fn rising(
        &self,
        limit: u32,
        options: Option<FeedOption>,
    ) -> Result<Submissions, RouxError> {
        self.get_feed("rising", limit, options).await
    }

    /// Get top posts.
    #[maybe_async::maybe_async]
    pub async fn top(
        &self,
        limit: u32,
        options: Option<FeedOption>,
    ) -> Result<Submissions, RouxError> {
        self.get_feed("top", limit, options).await
    }

    /// Get latest posts.
    #[maybe_async::maybe_async]
    pub async fn latest(
        &self,
        limit: u32,
        options: Option<FeedOption>,
    ) -> Result<Submissions, RouxError> {
        self.get_feed("new", limit, options).await
    }

    /// Get latest comments.
    #[maybe_async::maybe_async]
    pub async fn latest_comments(
        &self,
        depth: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Comments, RouxError> {
        self.get_comment_feed("comments", depth, limit).await
    }

    /// Get comments from article.
    #[maybe_async::maybe_async]
    pub async fn article_comments(
        &self,
        article: &str,
        depth: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Comments, RouxError> {
        self.get_comment_feed(&format!("comments/{}", article), depth, limit)
            .await
    }
}
