//! A read-only `User` class.
//! # Usage
//! ```rust,ignore
//! use roux::User;
//! let user = User::new("user_name");
//! ```
//! # Get Overview
//! ```rust,ignore
//! let overview = user.overview();
//! ```
//! # Get Submitted Posts
//! ```rust,ignore
//! let submitted = user.submitted();
//! ```
//! # Get Comments
//! ```rust,ignore
//! let comments = user.comments();
//! ```

extern crate reqwest;
extern crate serde_json;

use reqwest::Client;
use crate::util::RouxError;

mod responses;
use responses::{Comments, Overview, Submitted};

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
    pub fn overview(&self) -> Result<Overview, RouxError> {
        Ok(self.client
            .get(&format!(
                "https://www.reddit.com/user/{}/overview/.json",
                self.user
            ))
            .send()?
            .json::<Overview>()?)
    }

    /// Get user's submitted posts.
    pub fn submitted(&self) -> Result<Submitted, RouxError> {
        Ok(self.client.get(&format!(
                "https://www.reddit.com/user/{}/submitted/.json",
                self.user
            ))
            .send()?
            .json::<Submitted>()?)
    }

    /// Get user's submitted comments.
    pub fn comments(&self) -> Result<Comments, RouxError> {
        Ok(self.client.get(&format!(
                "https://www.reddit.com/user/{}/comments/.json",
                self.user
            ))
            .send()?
            .json::<Comments>()?)
    }
}

#[cfg(test)]
mod tests {
    use super::User;

    #[test]
    fn test_no_auth() {
        let user = User::new("beneater");

        // Test overview
        let overview = user.overview();
        assert!(overview.is_ok());

        // Test submitted
        let submitted = user.submitted();
        assert!(submitted.is_ok());

        // Test comments
        let comments = user.comments();
        assert!(comments.is_ok());
    }
}
