use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SubmittedChildrenData {
    subreddit: String,
    title: String,
    thumbnail: String,
    score: i32,
    created: f64,
    domain: String,
    is_self: bool,
}

#[derive(Debug, Deserialize)]
pub struct SubmittedChildren {
    kind: String,
    data: SubmittedChildrenData,
}

#[derive(Debug, Deserialize)]
pub struct SubmittedData {
    children: Vec<SubmittedChildren>,
}

#[derive(Debug, Deserialize)]
pub struct Submitted {
    kind: String,
    data: SubmittedData,
}
