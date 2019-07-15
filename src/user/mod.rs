extern crate reqwest;
extern crate serde_json;

use reqwest::Client;

pub mod structures;
use crate::util::RouxError;
use structures::{Comments, Overview, Submitted};

pub struct User {
    pub user: String,
    client: Client,
}

impl User {
    pub fn new(user: &str) -> User {
        User {
            user: user.to_owned(),
            client: Client::new(),
        }
    }

    pub fn overview(&self) -> Result<Overview, RouxError> {
        Ok(self.client
            .get(&format!(
                "https://www.reddit.com/user/{}/overview/.json",
                self.user
            ))
            .send()?
            .json::<Overview>()?)
    }

    pub fn submitted(&self) -> Result<Submitted, RouxError> {
        Ok(self.client.get(&format!(
                "https://www.reddit.com/user/{}/submitted/.json",
                self.user
            ))
            .send()?
            .json::<Submitted>()?)
    }

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
