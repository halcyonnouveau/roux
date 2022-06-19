//! # User Overview Responses
use crate::{responses::BasicThing};
use serde::Deserialize;

/// SubredditData
#[derive(Debug, Deserialize)]
pub struct SubredditData {
}


/// AboutData
#[derive(Debug, Deserialize)]
pub struct AboutData {
    ///snoovatar_img
    pub snoovatar_img: String,

    ///icon_img
    pub icon_img: String
}

/// Overview
pub type About = BasicThing<AboutData>;
