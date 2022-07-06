

//! # Inbox Reponses
use serde::Deserialize;
/// The response from an add friend request
#[derive(Debug, Deserialize)]
pub struct Friend {
    /// Was the friend request a success
    pub success: bool,
}