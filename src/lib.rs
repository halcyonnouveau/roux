use serde::Deserialize;

use reqwest::Client;
use reqwest::header::USER_AGENT;

pub mod util;

mod config;
mod me;
pub mod subreddit;
pub mod user;

pub struct Reddit {
    config: config::Config,
    client: Client,
}

#[derive(Deserialize, Debug)]
pub struct AuthData {
    pub access_token: String,
}

impl Reddit {
    pub fn new(client_id: &str, client_secret: &str) -> Reddit {
        Reddit {
            config: config::Config::new(&client_id, &client_secret),
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

    pub fn login(self) -> Result<me::Me, util::RouxError> {
        let url = "https://www.reddit.com/api/v1/access_token";
        let form = [
            ("grant_type", "password"),
            ("username", &self.config.username.to_owned().unwrap()),
            ("password", &self.config.password.to_owned().unwrap()),
        ];

        let request = self.client
            .post(url)
            // TODO get agent from env vars
            .header(USER_AGENT, "script:roux:v0.1.0 (by /u/beanpup_py)")
            .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
            .form(&form);

        let mut response = request.send().unwrap();

        if response.status() == 200 {
            let auth_data = response.json::<AuthData>().unwrap();
            Ok(me::Me::new(&auth_data.access_token, self.config))
        } else {
            Err(util::RouxError::Status(response.status()))
        }
    }
}
