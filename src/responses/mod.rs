use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BasicThing<T> {
    // An identifier that specifies the type of object that this is.
    // The valid kinds are:
    // - t1_ - Comment
    // - t2_ - Account
    // - t3_ - Link
    // - t4_ - Message
    // - t5_ - Subreddit
    // - t6_ - Award
    // - t8_ - PromoCampaign
    pub kind: String,
    // The data contained by this struct. This will vary depending on the type parameter
    // because each endpoint returns different contents.
    pub data: T,
}

#[derive(Deserialize, Debug)]
pub struct Listing<T> {
    pub modhash: Option<String>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub children: Vec<T>,
}

// Often times a basic thing will have this structure
pub type BasicListing<T> = BasicThing<Listing<BasicThing<T>>>;
