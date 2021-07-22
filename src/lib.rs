#![deny(missing_docs)]

//! # roux.rs
//! This crate provides simple access to the Reddit API.
//!
//! ## Using OAuth
//! To create an OAuth client with the reddit API, use the `Reddit` class.
//! ```no_run
//! use roux::Reddit;
//! # use tokio_test;
//!
//! # tokio_test::block_on(async {
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .username("USERNAME")
//!     .password("PASSWORD")
//!     .login()
//!     .await;
//!
//! let me = client.unwrap();
//! # })
//! ```
//! It is important that you pick a good user agent. The ideal format is
//! `platform:program:version (by /u/yourname)`, e.g. `macos:roux:v0.3.0 (by /u/beanpup_py)`.
//!
//! This will authticate you as the user given in the username function.
//!
//! ## Usage
//! Using the OAuth client, you can:
//!
//! ### Submit A Text Post
//! ```no_run
//! use roux::Reddit;
//! # use tokio_test;
//!
//! # tokio_test::block_on(async {
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .username("USERNAME")
//!     .password("PASSWORD")
//!     .login()
//!     .await;
//!
//! let me = client.unwrap();
//!
//! me.submit_text("TEXT_TITLE", "TEXT_BODY", "SUBREDDIT");
//! # })
//! ```
//!
//! ### Submit A Link Post
//! ```no_run
//! use roux::Reddit;
//! # use tokio_test;
//!
//! # tokio_test::block_on(async {
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .username("USERNAME")
//!     .password("PASSWORD")
//!     .login()
//!     .await;
//!
//! let me = client.unwrap();
//!
//! me.submit_link("LINK_TITLE", "LINK", "SUBREDDIT");
//! # })
//! ```

use serde::Deserialize;

use reqwest::header::USER_AGENT;
use reqwest::Client;

mod config;

/// Subreddit module.
pub mod subreddit;
pub use subreddit::{Subreddit, Subreddits};

/// User module.
pub mod user;
pub use user::User;

/// Me module.
pub mod me;
pub use me::Me;

pub mod responses;

/// Utils for requests.
pub mod util;
use util::url;

/// Client to use OAuth with Reddit.
pub struct Reddit {
    config: config::Config,
    client: Client,
}

#[derive(Deserialize, Debug)]
struct AuthData {
    pub access_token: String,
}

impl Reddit {
    /// Creates a `Reddit` instance with user_agent, client_id, and client_secret.
    pub fn new(user_agent: &str, client_id: &str, client_secret: &str) -> Reddit {
        Reddit {
            config: config::Config::new(user_agent, client_id, client_secret),
            client: Client::new(),
        }
    }

    /// Sets username.
    pub fn username(mut self, username: &str) -> Reddit {
        self.config.username = Some(username.to_owned());
        self
    }

    /// Sets password.
    pub fn password(mut self, password: &str) -> Reddit {
        self.config.password = Some(password.to_owned());
        self
    }

    /// Login as a user.
    pub async fn login(self) -> Result<me::Me, util::RouxError> {
        let url = &url::build_url("api/v1/access_token")[..];
        let form = [
            ("grant_type", "password"),
            ("username", &self.config.username.to_owned().unwrap()),
            ("password", &self.config.password.to_owned().unwrap()),
        ];

        let request = self
            .client
            .post(url)
            .header(USER_AGENT, &self.config.user_agent[..])
            .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
            .form(&form);

        let response = request.send().await?;

        if response.status() == 200 {
            let auth_data = response.json::<AuthData>().await.unwrap();
            Ok(me::Me::new(&auth_data.access_token, self.config))
        } else {
            Err(util::RouxError::Status(response))
        }
    }
}
