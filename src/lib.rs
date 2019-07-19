#![deny(missing_docs)]

//! # roux.rs
//! This crate simple access to he Reddit API
//! ## Using OAuth
//! To create an OAuth client with the reddit API, use the `Reddit` class.
//! ```rust,no_run
//! use roux::Reddit;
//!
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .username("USERNAME")
//!     .password("PASSWORD")
//!     .login();
//!
//! let me = client.unwrap();
//! ```
//! It is important that you pick a good user agent. The ideal format is
//! `platform:program:version (by /u/yourname)`, e.g. `linux:rawr:v0.0.1 (by /u/Aurora0001)`.
//!
//! This will authticate you as the user given in the username function.

use serde::Deserialize;

use reqwest::header::USER_AGENT;
use reqwest::Client;

/// Utilities.
pub mod util;
/// Deserialized API responses.
pub mod responses;
/// Read only subreddit.
pub mod subreddit;
/// Read only user.
pub mod user;

mod config;
mod me;

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
            config: config::Config::new(&user_agent, &client_id, &client_secret),
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
    pub fn login(self) -> Result<me::Me, util::RouxError> {
        let url = &url::build_url("api/v1/access_token")[..];
        let form = [
            ("grant_type", "password"),
            ("username", &self.config.username.to_owned().unwrap()),
            ("password", &self.config.password.to_owned().unwrap()),
        ];

        let request = self.client
            .post(url)
            .header(USER_AGENT, &self.config.user_agent[..])
            .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
            .form(&form);

        let mut response = request.send()?;

        if response.status() == 200 {
            let auth_data = response.json::<AuthData>().unwrap();
            Ok(me::Me::new(&auth_data.access_token, self.config))
        } else {
            Err(util::RouxError::Status(response))
        }
    }
}
