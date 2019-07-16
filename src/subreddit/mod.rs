extern crate reqwest;
extern crate serde_json;

use reqwest::Client;

pub mod responses;
use crate::util::RouxError;
use responses::{Moderators, Submissions};

pub struct Subreddit {
    pub name: String,
    url: String,
    client: Client,
}

impl Subreddit {
    pub fn new(name: &str) -> Subreddit {
        let subreddit_url = format!("https://www.reddit.com/r/{}", name);

        Subreddit {
            name: name.to_owned(),
            url: subreddit_url,
            client: Client::new(),
        }
    }

    pub fn moderators(&self) -> Result<Moderators, RouxError> {
        Ok(self.client
            .get(&format!("{}/about/moderators/.json", self.url))
            .send()?
            .json::<Moderators>()?)
    }

    fn get_feed(&self, ty: &str, limit: u32) -> Result<Submissions, RouxError> {
        Ok(self.client
            .get(&format!("{}/{}.json?limit={}", self.url, ty, limit))
            .send()?
            .json::<Submissions>()?)
    }

    pub fn hot(&self, limit: u32) -> Result<Submissions, RouxError> {
        self.get_feed("hot", limit)
    }

    pub fn rising(&self, limit: u32) -> Result<Submissions, RouxError> {
        self.get_feed("rising", limit)
    }

    pub fn top(&self, limit: u32) -> Result<Submissions, RouxError> {
        // TODO: time filter
        self.get_feed("top", limit)
    }
}

#[cfg(test)]
mod tests {
    use super::Subreddit;

    #[test]
    fn test_no_auth() {
        let subreddit = Subreddit::new("rust");

        // Test moderators
        let moderators = subreddit.moderators();
        assert!(moderators.is_ok());

        // Test feeds
        let hot = subreddit.hot(25);
        assert!(hot.is_ok());
        let rising = subreddit.rising(25);
        assert!(rising.is_ok());
        let top = subreddit.top(25);
        assert!(top.is_ok());
    }
}
