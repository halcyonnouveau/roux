#![deny(missing_docs)]

//! # roux.rs
//! This crate provides simple access to the Reddit API.
//!
//! ## Using OAuth
//! To create an OAuth client with the reddit API, use the `Reddit` class.
//! ```no_run
//! use roux::Reddit;
//! #[cfg(not(feature = "blocking"))]
//! use tokio;
//!
//! #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! #[maybe_async::maybe_async]
//! async fn main() {
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .username("USERNAME")
//!     .password("PASSWORD")
//!     .login()
//!     .await;
//! let me = client.unwrap();
//! }
//! ```
//!
//! It is important that you pick a good user agent. The ideal format is
//! `platform:program:version (by /u/yourname)`, e.g. `macos:roux:v0.3.0 (by /u/beanpup_py)`.
//!
//! This will authticate you as the user given in the username function.
//!
//!
//! ## Usage
//! Using the OAuth client, you can:
//!
//! ### Submit A Text Post
//! ```no_run
//! use roux::Reddit;
//! #[cfg(not(feature = "blocking"))]
//! use tokio;
//!
//! #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! #[maybe_async::maybe_async]
//! async fn main() {
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .username("USERNAME")
//!     .password("PASSWORD")
//!     .login()
//!     .await;
//! let me = client.unwrap();
//!
//! me.submit_text("TEXT_TITLE", "TEXT_BODY", "SUBREDDIT");
//! }
//! ```
//!
//! ### Submit A Link Post
//! ```no_run
//! use roux::Reddit;
//! #[cfg(not(feature = "blocking"))]
//! use tokio;
//!
//! #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! #[maybe_async::maybe_async]
//! async fn main() {
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .username("USERNAME")
//!     .password("PASSWORD")
//!     .login()
//!     .await;
//! let me = client.unwrap();
//!
//! me.submit_link("LINK_TITLE", "LINK", "SUBREDDIT");
//! }
//! ```

use serde::Deserialize;

use reqwest::header;
use reqwest::header::USER_AGENT;

mod config;

mod client;
use client::Client;

mod models;
pub use models::*;

/// Utils for requests.
pub mod util;
use util::url;

/// Client to use OAuth with Reddit.
pub struct Reddit {
    config: config::Config,
    client: Client,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum AuthResponse {
    AuthData { access_token: String },
    ErrorData { error: String },
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

    #[maybe_async::maybe_async]
    async fn create_client(mut self) -> Result<Reddit, util::RouxError> {
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
            let auth_data = response.json::<AuthResponse>().await?;

            let access_token = match auth_data {
                AuthResponse::AuthData { access_token } => access_token,
                AuthResponse::ErrorData { error } => return Err(util::RouxError::Auth(error)),
            };
            let mut headers = header::HeaderMap::new();

            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
            );

            headers.insert(
                header::USER_AGENT,
                header::HeaderValue::from_str(&self.config.user_agent[..]).unwrap(),
            );

            self.config.access_token = Some(access_token);
            self.client = Client::builder().default_headers(headers).build().unwrap();

            Ok(self)
        } else {
            Err(util::RouxError::Status(response))
        }
    }

    /// Login as a user.
    #[maybe_async::maybe_async]
    pub async fn login(self) -> Result<me::Me, util::RouxError> {
        let reddit = self.create_client().await?;
        Ok(me::Me::new(&reddit.config, &reddit.client))
    }

    /// Create a new authenticated `Subreddit` instance.
    #[maybe_async::maybe_async]
    pub async fn subreddit(self, name: &str) -> Result<models::Subreddit, util::RouxError> {
        let reddit = self.create_client().await?;
        Ok(models::Subreddit::new_oauth(name, &reddit.client))
    }
}
