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
    pub id: String,
    pub subject: String,
    pub was_comment: bool,
    pub author: Option<String>,
    pub parent_id: Option<String>,
    pub subreddit_name_prefixed: Option<String>,
    pub new: bool,
    pub r#type: String,
    pub body: String,
    pub dest: String,
    pub body_html: String,
    pub name: String,
    pub created: f64,
    pub created_utc: f64,
    pub context: String,
}
