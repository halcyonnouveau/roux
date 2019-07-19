use reqwest;
use serde_json;

/// Error type that occurs when an API request fails for some reason.
#[derive(Debug)]
pub enum RouxError {
    /// Occurs when the API has returned a non-success error code.
    Status(reqwest::Response),
    /// Occurs if the HTTP response from Reddit was corrupt and
    /// reqwest could not parse it.
    Network(reqwest::Error),
    /// Occurs if serde could not Deserialize the response.
    Parse(serde_json::Error),
}

impl From<reqwest::Error> for RouxError {
    fn from(e: reqwest::Error) -> Self {
        RouxError::Network(e)
    }
}

impl From<serde_json::Error> for RouxError {
    fn from(e: serde_json::Error) -> Self {
        RouxError::Parse(e)
    }
}
