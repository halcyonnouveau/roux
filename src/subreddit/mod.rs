extern crate reqwest;
extern crate serde_json;

use reqwest::Client;

pub mod structures;
use structures::Moderators;
use crate::util::RouxError;

pub struct Subreddit {
    pub subreddit: String,
    client: Client,
}

impl Subreddit {
    pub fn new(subreddit: &str) -> Subreddit {
        Subreddit {
            subreddit: subreddit.to_owned(),
            client: Client::new(),
        }
    }

    pub fn moderators(&self) -> Result<Moderators, RouxError> {
        match self.client
            .get(&format!(
                "https://www.reddit.com/r/{}/about/moderators/.json",
                self.subreddit
            ))
            .send()
        {
            Ok(mut res) => Ok(res.json::<Moderators>().unwrap()),
            Err(e) => Err(e.into()),
        }
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
    }
}
