//! # Subreddit Responses
use serde::Deserialize;

/// SubredditResponse
#[derive(Debug, Deserialize)]
pub struct SubredditResponse {
    /// Should be "t5" for subreddits.
    pub kind: Option<String>,
    /// Data about subreddit.
    pub data: SubredditData,
}

/// SubredditData
#[derive(Debug, Deserialize)]
pub struct SubredditData {
    /// User flair backgroud color
    pub user_flair_background_color: Option<String>,
    /// Submit text HTMl
    pub submit_text_html: Option<String>,
    /// Restrict posting
    pub restrict_posting: Option<bool>,
    /// User is banned
    pub user_is_banned: Option<bool>,
    /// Free form reports
    pub free_form_reports: Option<bool>,
    /// Wiki enabled
    pub wiki_enabled: Option<bool>,
    /// User is muted
    pub user_is_muted: Option<bool>,
    /// User can flair in subreddit
    pub user_can_flair_in_sr: Option<bool>,
    /// Display name
    pub display_name: Option<String>,
    /// Header image
    pub header_img: Option<String>,
    /// Title
    pub title: Option<String>,
    /// Allow galleries
    pub allow_galleries: Option<bool>,
    /// Icon size
    pub icon_size: Option<Vec<u32>>,
    /// Primary color
    pub primary_color: Option<String>,
    /// Active user count
    pub active_user_count: Option<u32>,
    /// Icon image
    pub icon_img: Option<String>,
    /// Display name prefixed
    pub display_name_prefixed: Option<String>,
    /// Number of accounts active
    pub accounts_active: Option<u32>,
    /// Public traffic
    pub public_traffic: Option<bool>,
    /// Subscriber count
    pub subscribers: Option<u32>,
    /// User flair richtext
    pub user_flair_richtext: Option<Vec<String>>,
    /// Videostream links count
    pub videostream_links_count: Option<u32>,
    /// Subreddit name
    pub name: Option<String>,
    /// Quarantine
    pub quarantine: Option<bool>,
    /// Hide ads
    pub hide_ads: Option<bool>,
    /// Prediction leaderboard entry type
    pub prediction_leaderboard_entry_type: Option<String>,
    /// Emojis enabled
    pub emojis_enabled: Option<bool>,
    /// Advertiser category
    pub advertiser_category: Option<String>,
    /// Public description
    pub public_description: Option<String>,
    /// Comment score hide mins
    pub comment_score_hide_mins: Option<u32>,
    /// Allow predictions
    pub allow_predictions: Option<bool>,
    /// User has favorited
    pub user_has_favorited: Option<bool>,
    /// User flair template ID
    pub user_flair_template_id: Option<String>,
    /// Community icon
    pub community_icon: Option<String>,
    /// Banner background image
    pub banner_background_image: Option<String>,
    /// Original content tag enabled
    pub original_content_tag_enabled: Option<bool>,
    /// Submit text
    pub submit_text: Option<String>,
    /// Description HTML
    pub description_html: Option<String>,
    /// Spoilers enabled
    pub spoilers_enabled: Option<bool>,
    /// Header title
    pub header_title: Option<String>,
    /// Header size
    pub header_size: Option<Vec<u32>>,
    /// User flair position
    pub user_flair_position: Option<String>,
    /// All original content
    pub all_original_content: Option<bool>,
    /// Has menu widget
    pub has_menu_widget: Option<bool>,
    /// Is enrolled in new modmail
    pub is_enrolled_in_new_modmail: Option<bool>,
    /// Key color
    pub key_color: Option<String>,
    /// Can assign user flair
    pub can_assign_user_flair: Option<bool>,
    /// Created
    pub created: Option<f64>,
    /// WLS?
    pub wls: Option<u32>,
    /// Show media preview
    pub show_media_preview: Option<bool>,
    /// Submission type
    pub submission_type: Option<String>,
    /// User is subscriber
    pub user_is_subscriber: Option<bool>,
    /// Disable contributor requests
    pub disable_contributor_requests: Option<bool>,
    /// Allow videogifs
    pub allow_videogifs: Option<bool>,
    /// User flair type
    pub user_flair_type: Option<String>,
    /// Allow polls
    pub allow_polls: Option<bool>,
    /// Collapse deleted comments
    pub collapse_deleted_comments: Option<bool>,
    /// Emojis custom size
    pub emojis_custom_size: Option<Vec<u32>>,
    /// Public description HTML
    pub public_description_html: Option<String>,
    /// Allow videos
    pub allow_videos: Option<bool>,
    /// Is crosspostable subreddit
    pub is_crosspostable_subreddit: Option<bool>,
    /// Notification level
    pub notification_level: Option<u32>,
    /// Can assign link flair
    pub can_assign_link_flair: Option<bool>,
    /// Accounts active is fuzzed
    pub accounts_active_is_fuzzed: Option<bool>,
    /// Submit text label
    pub submit_text_label: Option<String>,
    /// Link flair position
    pub link_flair_position: Option<String>,
    /// User subreddit flair enabled
    pub user_sr_flair_enabled: Option<bool>,
    /// User flair enabled in subreddit
    pub user_flair_enabled_in_sr: Option<bool>,
    /// Allow chat post creation
    pub allow_chat_post_creation: Option<bool>,
    /// Allow discovery
    pub allow_discovery: Option<bool>,
    /// User subreddit theme enabled
    pub user_sr_theme_enabled: Option<bool>,
    /// Link flair enabled
    pub link_flair_enabled: Option<bool>,
    /// Subreddit type
    pub subreddit_type: Option<String>,
    /// Suggested comment sort
    pub suggested_comment_sort: Option<String>,
    /// Banner image
    pub banner_img: Option<String>,
    /// User flair text
    pub user_flair_text: Option<String>,
    /// Banner background color
    pub banner_background_color: Option<String>,
    /// Show media
    pub show_media: Option<bool>,
    /// Subreddit ID
    pub id: Option<String>,
    /// User is moderator
    pub user_is_moderator: Option<bool>,
    /// Over 18
    pub over18: Option<bool>,
    /// Description
    pub description: Option<String>,
    /// Is chat post feature enabled
    pub is_chat_post_feature_enabled: Option<bool>,
    /// Submit link label
    pub submit_link_label: Option<String>,
    /// User flair text color
    pub user_flair_text_color: Option<String>,
    /// Restrict commenting
    pub restrict_commenting: Option<bool>,
    /// User flair CSS class
    pub user_flair_css_class: Option<String>,
    /// Allow images
    pub allow_images: Option<bool>,
    /// Language
    pub lang: Option<String>,
    /// Whitelist status
    pub whitelist_status: Option<String>,
    /// URL
    pub url: Option<String>,
    /// Created UTC
    pub created_utc: Option<f64>,
    /// Banner size
    pub banner_size: Option<Vec<u32>>,
    /// Mobile banner image
    pub mobile_banner_image: Option<String>,
    /// User is contributor
    pub user_is_contributor: Option<bool>,
    /// Allow predictions tournament
    pub allow_predictions_tournament: Option<bool>,
}
