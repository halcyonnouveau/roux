use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CommentsChildrenData {
    body: String,
    body_html: String,
    link_title: String,
    link_url: String,
    subreddit: String,
    created: f64,
}

#[derive(Debug, Deserialize)]
pub struct CommentsChildren {
    kind: String,
    data: CommentsChildrenData,
}

#[derive(Debug, Deserialize)]
pub struct CommentsData {
    children: Vec<CommentsChildren>,
}

#[derive(Debug, Deserialize)]
pub struct Comments {
    kind: String,
    data: CommentsData,
}
