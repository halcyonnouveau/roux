use crate::client::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

/// String function for serde defaults.
pub fn default_string() -> String {
    "".to_string()
}

/// Default headers must contain user agent
pub fn default_headers() -> HeaderMap {
    vec![(USER_AGENT, HeaderValue::from_static("roux/rust"))]
        .into_iter()
        .collect()
}

/// Default client
pub fn default_client() -> Client {
    ClientBuilder::new()
        .default_headers(default_headers())
        .build()
        .expect("Error creating default client ")
}
