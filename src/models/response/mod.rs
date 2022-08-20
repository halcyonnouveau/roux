//! # Responses
//! Base responses

use serde::{Deserialize, Serialize};

/// Basic structure of a Reddit response.
/// See: <https://github.com/reddit-archive/reddit/wiki/JSON>
#[derive(Serialize, Deserialize, Debug)]
pub struct BasicThing<T> {
    /// An identifier that specifies the type of object that this is.
    pub kind: String,
    /// The data contained by this struct. This will vary depending on the type parameter
    /// because each endpoint returns different contents.
    pub data: T,
}

/// JSON list response.
#[derive(Serialize, Deserialize, Debug)]
pub struct Listing<T> {
    /// Modhash
    pub modhash: Option<String>,
    /// The number of children in the listing.
    pub dist: Option<i32>,
    /// The fullname of the listing that follows after this page.
    pub after: Option<String>,
    /// The fullname of the listing that follows before this page.
    pub before: Option<String>,
    /// A list of `things` that this Listing wraps.
    pub children: Vec<T>,
}

/// Often times a basic thing will have this structure.
pub type BasicListing<T> = BasicThing<Listing<BasicThing<T>>>;
