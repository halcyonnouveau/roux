use serde::Deserialize;

use reqwest::header::USER_AGENT;
use reqwest::Client;

pub mod util;
pub mod responses;
pub mod subreddit;
pub mod user;

mod config;
mod me;

use util::url;

pub struct Reddit {
    config: config::Config,
    client: Client,
}

#[derive(Deserialize, Debug)]
struct AuthData {
    pub access_token: String,
}

impl Reddit {
    pub fn new(user_agent: &str, client_id: &str, client_secret: &str) -> Reddit {
        Reddit {
            config: config::Config::new(&user_agent, &client_id, &client_secret),
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
