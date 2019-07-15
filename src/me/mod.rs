extern crate reqwest;
extern crate serde_json;

use reqwest::header;
use reqwest::Client;

pub mod structures;

use structures::MeData;
use crate::util::RouxError;
use crate::config::Config;

pub struct Me {
    pub access_token: String,
    client: Client,
    config: Config,
}

impl Me {
    pub fn new(access_token: &str, config: Config) -> Me {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap()
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Me {
            access_token: access_token.to_owned(),
            config: config,
            client: client
        }
    }

    pub fn get(&self) -> Result<MeData, RouxError> {
        match self.client
            .get("https://oauth.reddit.com/api/v1/me/.json")
            .send()
        {
            Ok(mut res) => Ok(res.json::<MeData>().unwrap()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn logout(self) -> Result<(), RouxError> {
        let url = "https://www.reddit.com/api/v1/revoke_token";

        let form = [
            ("access_token", self.access_token.to_owned()),
        ];

        let request = self.client
            .post(url)
            .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
            .form(&form);

        let response = request.send().unwrap();

        if response.status() == 204 {
            Ok(())
        } else {
            Err(RouxError::Status(response.status()))
        }
    }
}
