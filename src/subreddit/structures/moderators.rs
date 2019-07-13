use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ModeratorsChildren {
    id: String,
    name: String,
    author_flair_text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModeratorsData {
    children: Vec<ModeratorsChildren>,
}

#[derive(Debug, Deserialize)]
pub struct Moderators {
    kind: String,
    data: ModeratorsData,
}
