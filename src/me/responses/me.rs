use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MeData {
    pub id: String,
    pub is_employee: bool,
    pub verified: bool,
    pub over_18: bool,
    pub has_verified_email: bool,
    pub is_suspended: bool,
    pub has_mail: bool,
    pub inbox_count: f64,
    pub created: f64,
    pub created_utc: f64,
    pub in_beta: bool,
    pub comment_karma: i32,
    pub link_karma: i32,
    pub is_mod: bool,
    pub is_gold: bool,
    pub icon_img: String,
}

#[derive(Debug, Deserialize)]
pub struct InboxItem {
    id: String,
    subject: String,
    was_comment: bool,
    author: String,
    parent_id: Option<String>,
    subreddit_name_prefixed: Option<String>,
    new: bool,
    r#type: String,
    body: String,
    dest: String,
    body_html: String,
    name: String,
    created: f64,
    created_utc: f64,
    context: String,
}
