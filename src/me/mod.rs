//! # Me
//! Me module.

extern crate reqwest;
extern crate serde_json;

use reqwest::{header};
use crate::client::{Client, Response};
use serde::Serialize;

use crate::config::Config;
use crate::util::{url, FeedOption, RouxError};

pub mod responses;

use responses::{Friend, Inbox, MeData, Saved};

/// Me
#[derive(Debug, Clone)]
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

    #[maybe_async::maybe_async]
    async fn get(&self, url: &str) -> Result<Response, RouxError> {
        let get_url = url::build_oauth(url);

        match self.client.get(&get_url[..]).send().await {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        }
    }

    #[maybe_async::maybe_async]
    async fn post<T: Serialize>(&self, url: &str, form: T) -> Result<Response, RouxError> {
        let post_url = url::build_oauth(url).to_owned();

        match self.client.post(&post_url[..]).form(&form).send().await {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        }
    }

    /// Get me
    #[maybe_async::maybe_async]
    pub async fn me(&self) -> Result<MeData, RouxError> {
        match self.get("api/v1/me").await {
            Ok(res) => Ok(res.json::<MeData>().await?),
            Err(e) => Err(e),
        }
    }

    /// Submit link
    #[maybe_async::maybe_async]
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
    #[maybe_async::maybe_async]
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

    /// Adds a friend to a subreddit with the specified type
    #[maybe_async::maybe_async]
    pub async fn add_subreddit_friend(
        &self,
        username: &str,
        typ: &str,
        sub: &str,
    ) -> Result<bool, RouxError> {
        let form = [("name", username), ("type", typ)];
        Ok(self
            .post(format!("r/{}/api/friend", sub).as_str(), form)
            .await?
            .json::<Friend>()
            .await?
            .success)
    }

    /// Removes a friend to a subreddit with the specified type
    #[maybe_async::maybe_async]
    pub async fn remove_subreddit_friend(
        &self,
        username: &str,
        typ: &str,
        sub: &str,
    ) -> Result<bool, RouxError> {
        let form = [("name", username), ("type", typ)];
        Ok(self
            .post(format!("r/{}/api/unfriend", sub).as_str(), form)
            .await?
            .json::<Friend>()
            .await?
            .success)
    }

    /// Compose message
    #[maybe_async::maybe_async]
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
    #[maybe_async::maybe_async]
    pub async fn inbox(&self) -> Result<Inbox, RouxError> {
        Ok(self.get("message/inbox").await?.json::<Inbox>().await?)
    }

    /// Get saved
    #[maybe_async::maybe_async]
    pub async fn saved(&self, options: Option<FeedOption>) -> Result<Saved, RouxError> {
        let url = &mut format!(
            "user/{}/saved/.json",
            self.config.username.to_owned().unwrap()
        );

        if let Some(options) = options {
            options.build_url(url);
        }

        Ok(self.get(&url).await?.json::<Saved>().await?)
    }

    /// Get upvoted
    #[maybe_async::maybe_async]
    pub async fn upvoted(&self, options: Option<FeedOption>) -> Result<Saved, RouxError> {
        let url = &mut format!(
            "user/{}/upvoted/.json",
            self.config.username.to_owned().unwrap()
        );

        if let Some(options) = options {
            options.build_url(url);
        }

        Ok(self.get(&url).await?.json::<Saved>().await?)
    }

    /// Get downvoted
    #[maybe_async::maybe_async]
    pub async fn downvoted(&self, options: Option<FeedOption>) -> Result<Saved, RouxError> {
        let url = &mut format!(
            "user/{}/downvoted/.json",
            self.config.username.to_owned().unwrap()
        );

        if let Some(options) = options {
            options.build_url(url);
        }

        Ok(self.get(&url).await?.json::<Saved>().await?)
    }

    /// Get users unread messages
    #[maybe_async::maybe_async]
    pub async fn unread(&self) -> Result<Inbox, RouxError> {
        Ok(self.get("message/unread").await?.json::<Inbox>().await?)
    }

    /// Mark messages as read
    #[maybe_async::maybe_async]
    pub async fn mark_read(&self, ids: &str) -> Result<Response, RouxError> {
        let form = [("id", ids)];
        self.post("api/read_message", &form).await
    }

    /// Mark messages as unread
    #[maybe_async::maybe_async]
    pub async fn mark_unread(&self, ids: &str) -> Result<Response, RouxError> {
        let form = [("id", ids)];
        self.post("api/unread_message", &form).await
    }

    /// Comment
    #[maybe_async::maybe_async]
    pub async fn comment(&self, text: &str, parent: &str) -> Result<Response, RouxError> {
        let form = [("text", text), ("parent", parent)];
        self.post("api/comment", &form).await
    }

    /// Edit a 'thing'
    #[maybe_async::maybe_async]
    pub async fn edit(&self, text: &str, parent: &str) -> Result<Response, RouxError> {
        let form = [("text", text), ("thing_id", parent)];
        self.post("api/editusertext", &form).await
    }

    /// Logout
    #[maybe_async::maybe_async]
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
