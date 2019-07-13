extern crate reqwest;
extern crate serde_json;

use reqwest::Client;

pub mod structures;
use structures::{Comments, Overview, Submitted};
use crate::util::RouxError;

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
        match self.client
            .get(&format!(
                "https://www.reddit.com/user/{}/overview/.json",
                self.user
            ))
            .send()
        {
            Ok(mut res) => Ok(res.json::<Overview>().unwrap()),
            Err(e) => Err(e.into()),
        }

    }

    pub fn submitted(&self) -> Result<Submitted, RouxError> {
        match self.client
            .get(&format!(
                "https://www.reddit.com/user/{}/submitted/.json",
                self.user
            ))
            .send()
        {
            Ok(mut res) => Ok(res.json::<Submitted>().unwrap()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn comments(&self) -> Result<Comments, RouxError> {
        match self.client
            .get(&format!(
                "https://www.reddit.com/user/{}/comments/.json",
                self.user
            ))
            .send()
        {
            Ok(mut res) => Ok(res.json::<Comments>().unwrap()),
            Err(e) => Err(e.into()),
        }
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
