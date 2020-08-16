//! # User
//! A read-only module to read data from for a specific user.
//!
//! # Usage
//! ```should_fail
//! use roux::User;
//! let user = User::new("beanpup_py");
//! // Now you are able to:
//! // Get overview
//! let overview = user.overview().await;
//! // Get submitted posts.
//! let submitted = user.submitted().await;
//! // Get comments.
//! let comments = user.comments().await;
//! ```

extern crate reqwest;
extern crate serde_json;

use crate::util::RouxError;
use reqwest::Client;

pub mod responses;
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
    pub async fn overview(&self) -> Result<Overview, RouxError> {
        Ok(self
            .client
            .get(&format!(
                "https://www.reddit.com/user/{}/overview/.json",
                self.user
            ))
            .send()
            .await?
            .json::<Overview>()
            .await?)
    }

    /// Get user's submitted posts.
    pub async fn submitted(&self) -> Result<Submitted, RouxError> {
        Ok(self
            .client
            .get(&format!(
                "https://www.reddit.com/user/{}/submitted/.json",
                self.user
            ))
            .send()
            .await?
            .json::<Submitted>()
            .await?)
    }

    /// Get user's submitted comments.
    pub async fn comments(&self) -> Result<Comments, RouxError> {
        Ok(self
            .client
            .get(&format!(
                "https://www.reddit.com/user/{}/comments/.json",
                self.user
            ))
            .send()
            .await?
            .json::<Comments>()
            .await?)
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
