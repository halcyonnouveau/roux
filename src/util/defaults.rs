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
///
/// Compression is explicitly disabled to avoid Reddit's anti-bot fingerprinting,
/// which can flag the `Accept-Encoding` header that reqwest auto-adds when a
/// feature is enabled transitively by another dependency.
pub fn default_client() -> Client {
    ClientBuilder::new()
        .default_headers(default_headers())
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .no_zstd()
        .build()
        .expect("Error creating default client ")
}
