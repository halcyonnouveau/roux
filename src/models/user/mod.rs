//! # User
//! A read-only module to read data from for a specific user.
//!
//! Reddit no longer serves these endpoints to unauthenticated clients, so a
//! `User` should be created through [`Reddit::user`](crate::Reddit::user),
//! which authenticates with an application-only token when no username and
//! password are set.
//!
//! # Usage
//! ```no_run
//! use roux::Reddit;
//! use roux::util::FeedOption;
//! # #[cfg(not(feature = "blocking"))]
//! # use tokio;
//!
//! # #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! # #[maybe_async::maybe_async]
//! # async fn main() {
//! let user = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .user("kasuporo")
//!     .await
//!     .unwrap();
//! // Now you are able to:
//!
//! // Get overview
//! let overview = user.overview(None).await;
//!
//! // Get submitted posts.
//! let submitted = user.submitted(None).await;
//!
//! // Get comments.
//! let comments = user.comments(None).await;
//! # }
//! ```

extern crate serde_json;

use crate::client::Client;
use crate::util::defaults::default_client;
use crate::util::{FeedOption, RouxError};

use crate::models::{About, Comments, Overview, Submissions};

/// User.
pub struct User {
    /// User's name.
    pub user: String,
    url: String,
    client: Client,
}

impl User {
    /// Create a new unauthenticated `User` instance.
    ///
    /// Reddit blocks unauthenticated requests to these endpoints, so this
    /// will almost certainly fail with a 403. Use
    /// [`Reddit::user`](crate::Reddit::user) instead.
    #[deprecated(
        since = "2.3.0",
        note = "Reddit blocks unauthenticated API access; use `Reddit::user` instead"
    )]
    pub fn new(user: &str) -> User {
        User {
            user: user.to_owned(),
            url: format!("https://www.reddit.com/user/{}", user),
            client: default_client(),
        }
    }

    /// Create a new authenticated `User` instance using an oauth client
    /// from the `Reddit` module.
    pub fn new_oauth(user: &str, client: &Client) -> User {
        User {
            user: user.to_owned(),
            url: format!("https://oauth.reddit.com/user/{}", user),
            client: client.to_owned(),
        }
    }

    /// Get user's overview.
    #[maybe_async::maybe_async]
    pub async fn overview(&self, options: Option<FeedOption>) -> Result<Overview, RouxError> {
        let url = &mut format!("{}/overview/.json?", self.url);

        if let Some(options) = options {
            options.build_url(url);
        }

        Ok(self
            .client
            .get(&url.to_owned())
            .send()
            .await?
            .json::<Overview>()
            .await?)
    }

    /// Get user's submitted posts.
    #[maybe_async::maybe_async]
    pub async fn submitted(&self, options: Option<FeedOption>) -> Result<Submissions, RouxError> {
        let url = &mut format!("{}/submitted/.json?", self.url);

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

    /// Get user's submitted comments.
    #[maybe_async::maybe_async]
    pub async fn comments(&self, options: Option<FeedOption>) -> Result<Comments, RouxError> {
        let url = &mut format!("{}/comments/.json?", self.url);

        if let Some(options) = options {
            options.build_url(url);
        }

        Ok(self
            .client
            .get(&url.to_owned())
            .send()
            .await?
            .json::<Comments>()
            .await?)
    }

    /// Get user's about page
    #[maybe_async::maybe_async]
    pub async fn about(&self, options: Option<FeedOption>) -> Result<About, RouxError> {
        let url = &mut format!("{}/about/.json?", self.url);

        if let Some(options) = options {
            options.build_url(url);
        }

        Ok(self
            .client
            .get(&url.to_owned())
            .send()
            .await?
            .json::<About>()
            .await?)
    }
}
