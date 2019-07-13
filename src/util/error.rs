use reqwest;
use serde_json;

#[derive(Debug)]
pub enum RouxError {
    Network(reqwest::Error),
    Parse(serde_json::Error),
    Status(reqwest::StatusCode),
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
