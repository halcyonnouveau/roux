extern crate reqwest;
extern crate serde_json;

use reqwest::{header, Client, Response};
use serde::Serialize;

use crate::config::Config;
use crate::util::{url, RouxError};

mod responses;
use responses::MeData;

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
            header::HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
        );

        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&config.user_agent[..]).unwrap(),
        );

        let client = Client::builder().default_headers(headers).build().unwrap();

        Me {
            access_token: access_token.to_owned(),
            config: config,
            client: client,
        }
    }

    pub fn get(&self, url: &str) -> Result<Response, RouxError> {
        let get_url = url::build_oauth(url);

        match self.client.get(&get_url[..]).send() {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        }
    }

    pub fn me(&self) -> Result<MeData, RouxError> {
        match self.get("api/v1/me") {
            Ok(mut res) => Ok(res.json::<MeData>()?),
            Err(e) => Err(e.into()),
        }
    }

    pub fn post<T: Serialize>(&self, url: &str, form: T) -> Result<Response, RouxError> {
        let post_url = url::build_oauth(url).to_owned();

        match self.client.post(&post_url[..]).form(&form).send() {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        }
    }

    pub fn submit_link(&self, title: &str, link: &str, sr: &str) -> Result<Response, RouxError> {
        let form = [
            ("kind", "link"),
            ("title", title),
            ("url", link),
            ("sr", sr),
        ];

        self.post("api/submit", &form)
    }

    pub fn submit_text(&self, title: &str, text: &str, sr: &str) -> Result<Response, RouxError> {
        let form = [
            ("kind", "self"),
            ("title", title),
            ("text", text),
            ("sr", sr),
        ];

        self.post("api/submit", &form)
    }

    pub fn compose_message(
        &self,
        username: &str,
        subject: &str,
        body: &str,
    ) -> Result<Response, RouxError> {
        let form = [
            ("api_type", "json"),
            ("subject", subject),
            ("text", body),
            ("to", username),
        ];

        self.post("api/compose", &form)
    }

    pub fn logout(self) -> Result<(), RouxError> {
        let url = "https://www.reddit.com/api/v1/revoke_token";

        let form = [("access_token", self.access_token.to_owned())];

        let response = self
            .client
            .post(url)
            .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
            .form(&form)
            .send()?;

        if response.status() == 204 {
            Ok(())
        } else {
            Err(RouxError::Status(response))
        }
    }
}
