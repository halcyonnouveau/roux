use std::error;
use std::fmt;

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

impl fmt::Display for RouxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RouxError::Status(ref err) => write!(f, "Status error: {}", err.status()),
            RouxError::Network(ref err) => err.fmt(f),
            RouxError::Parse(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for RouxError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            RouxError::Status(_) => None,
            RouxError::Network(ref err) => Some(err),
            RouxError::Parse(ref err) => Some(err),
        }
    }
}
