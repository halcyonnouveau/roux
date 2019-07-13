use serde::Deserialize;

extern crate reqwest;
use reqwest::Client;
use reqwest::header::USER_AGENT;

pub mod subreddit;
pub mod user;

pub mod util;
use util::RouxError;

mod config;
use config::Config;

pub struct Reddit {
    config: Config,
    client: Client,
}

#[derive(Deserialize, Debug)]
pub struct AuthData {
    pub access_token: String,
}

impl Reddit {
    pub fn new(client_id: &str, client_secret: &str) -> Reddit {
        Reddit {
            config: Config::new(&client_id, &client_secret),
            client: Client::new(),
        }
    }

    pub fn username(mut self, username: &str) -> Reddit {
        self.config.username = Some(username.to_owned());
        self
    }

    pub fn password(mut self, password: &str) -> Reddit {
        self.config.password = Some(password.to_owned());
        self
    }

    pub fn login(mut self) -> Result<Reddit, RouxError> {
        let url = "https://www.reddit.com/api/v1/access_token";

        let body = format!(
            "grant_type=password&username={}&password={}",
            &self.config.username.to_owned().unwrap(),
            &self.config.password.to_owned().unwrap()
        );

        let request = self.client
            .post(url)
            .header(USER_AGENT, "Reqwest")
            .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
            .body(body);

        let mut result = request.send().unwrap();

        if result.status() == 200 {
            let auth_data = result.json::<AuthData>().unwrap();
            self.config.access_token = Some(auth_data.access_token);
            Ok(self)
        } else {
            Err(RouxError::Status(result.status()))
        }
    }
}
