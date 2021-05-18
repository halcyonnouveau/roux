//! # Me
//! Me module.

extern crate reqwest;
extern crate serde_json;

use reqwest::{header, Client, Response};
use serde::Serialize;

use crate::config::Config;
use crate::util::{url, RouxError};

pub mod responses;

use crate::subreddit::responses::Submissions;
use responses::{Inbox, MeData};
use crate::me::responses::me::Friend;
use self::reqwest::{Body, Url};

/// Me
pub struct Me {
    /// Access token
    pub access_token: String,
    client: Client,
    config: Config,
}

impl Me {
    /// Create a new `me`
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
            config,
            client,
        }
    }

    async fn get(&self, url: &str) -> Result<Response, RouxError> {
        let get_url = url::build_oauth(url);

        match self.client.get(&get_url[..]).send().await {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        }
    }

    async fn post<T: Serialize>(&self, url: &str, form: T) -> Result<Response, RouxError> {
        let post_url = url::build_oauth(url).to_owned();

        match self.client.post(&post_url[..]).form(&form).send().await {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        }
    }

    /// Get me
    pub async fn me(&self) -> Result<MeData, RouxError> {
        match self.get("api/v1/me").await {
            Ok(res) => Ok(res.json::<MeData>().await?),
            Err(e) => Err(e.into()),
        }
    }

    /// Submit link
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

    /// Submit text
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
    ///Adds a friend to a subreddit with the specified type
    pub async fn add_subreddit_friend(
        &self,
        username: &str,
        typ: &str,
        sub: &str,
    ) -> Result<bool, RouxError> {
        let form = [
            ("name", username),
            ("type", typ),
        ];
        Ok(self
            .post(format!("r/{}/api/friend", sub).as_str(), form)
            .await?.json::<Friend>().await?.success)
    }
    /// Compose message
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

    /// Get user's submitted posts.
    pub async fn inbox(&self) -> Result<Inbox, RouxError> {
        Ok(self.get("message/inbox").await?.json::<Inbox>().await?)
    }

    /// Get saved
    pub async fn saved(&self) -> Result<Submissions, RouxError> {
        let url = format!(
            "user/{}/saved/.json",
            self.config.username.to_owned().unwrap()
        );

        Ok(self.get(&url).await?.json::<Submissions>().await?)
    }

    /// Get upvoted
    pub async fn upvoted(&self) -> Result<Submissions, RouxError> {
        let url = format!(
            "user/{}/upvoted/.json",
            self.config.username.to_owned().unwrap()
        );

        Ok(self.get(&url).await?.json::<Submissions>().await?)
    }

    /// Get downvoted
    pub async fn downvoted(&self) -> Result<Submissions, RouxError> {
        let url = format!(
            "user/{}/downvoted/.json",
            self.config.username.to_owned().unwrap()
        );

        Ok(self.get(&url).await?.json::<Submissions>().await?)
    }

    /// Get users unread messages
    pub async fn unread(&self) -> Result<Inbox, RouxError> {
        Ok(self.get("message/unread").await?.json::<Inbox>().await?)
    }

    /// Mark messages as read
    pub async fn mark_read(&self, ids: &str) -> Result<Response, RouxError> {
        let form = [("id", ids)];
        self.post("api/read_message", &form).await
    }

    /// Mark messages as unread
    pub async fn mark_unread(&self, ids: &str) -> Result<Response, RouxError> {
        let form = [("id", ids)];
        self.post("api/unread_message", &form).await
    }

    /// Comment
    pub async fn comment(&self, text: &str, parent: &str) -> Result<Response, RouxError> {
        let form = [("text", text), ("parent", parent)];
        self.post("api/comment", &form).await
    }

    /// Edit
    pub async fn edit(&self, text: &str, parent: &str) -> Result<Response, RouxError> {
        let form = [("text", text), ("thing_id", parent)];
        self.post("api/editusertext", &form).await
    }

    /// Logout
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
