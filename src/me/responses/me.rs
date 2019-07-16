use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MeData {
    id: String,
    is_employee: bool,
    verified: bool,
    over_18: bool,
    has_verified_email: bool,
    is_suspended: bool,
    has_mail: bool,
    inbox_count: f64,
    created: f64,
    created_utc: f64,
    in_beta: bool,
    comment_karma: i32,
    link_karma: i32,
    is_mod: bool,
    is_gold: bool,
    icon_img: String,
}
