extern crate reqwest;
extern crate serde_json;

use reqwest::{header, Client, Response};
use serde::Serialize;

use crate::config::Config;
use crate::responses::BasicListing;
use crate::util::{url, RouxError};

mod responses;
use responses::InboxItem;
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

    pub async fn get(&self, url: &str) -> Result<Response, RouxError> {
        let get_url = url::build_oauth(url);

        match self.client.get(&get_url[..]).send().await {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn me(&self) -> Result<MeData, RouxError> {
        match self.get("api/v1/me").await {
            Ok(res) => Ok(res.json::<MeData>().await?),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn post<T: Serialize>(&self, url: &str, form: T) -> Result<Response, RouxError> {
        let post_url = url::build_oauth(url).to_owned();

        match self.client.post(&post_url[..]).form(&form).send().await {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn submit_link(
        &self,
        title: &str,
        link: &str,
        sr: &str,
    ) -> Result<Response, RouxError> {
        let form = [
            ("kind", "link"),
            ("title", title),
            ("url", link),
            ("sr", sr),
        ];

        self.post("api/submit", &form).await
    }

    pub async fn submit_text(
        &self,
        title: &str,
        text: &str,
        sr: &str,
    ) -> Result<Response, RouxError> {
        let form = [
            ("kind", "self"),
            ("title", title),
            ("text", text),
            ("sr", sr),
        ];

        self.post("api/submit", &form).await
    }

    pub async fn compose_message(
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

        self.post("api/compose", &form).await
    }

    // Get user's submitted posts.
    pub async fn inbox(&self) -> Result<BasicListing<InboxItem>, RouxError> {
        Ok(self
            .get("message/inbox")
            .await?
            .json::<BasicListing<InboxItem>>()
            .await?)
    }

    ///  Get users unread messages
    pub async fn unread(&self) -> Result<BasicListing<InboxItem>, RouxError> {
        Ok(self
            .get("message/unread")
            .await?
            .json::<BasicListing<InboxItem>>()
            .await?)
    }

    pub async fn mark_read(&self, ids: &str) -> Result<Response, RouxError> {
        let form = [("id", ids)];
        self.post("api/read_message", &form).await
    }

    pub async fn comment(&self, text: &str, parent: &str) -> Result<Response, RouxError> {
        let form = [("text", text), ("parent", parent)];
        self.post("api/comment", &form).await
    }

    pub async fn edit(&self, text: &str, parent: &str) -> Result<Response, RouxError> {
        let form = [("text", text), ("thing_id", parent)];
        self.post("api/editusertext", &form).await
    }

    pub async fn logout(self) -> Result<(), RouxError> {
        let url = "https://www.reddit.com/api/v1/revoke_token";

        let form = [("access_token", self.access_token.to_owned())];

        let response = self
            .client
            .post(url)
            .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
            .form(&form)
            .send()
            .await?;

        if response.status() == 204 {
            Ok(())
        } else {
            Err(RouxError::Status(response))
        }
    }
}
