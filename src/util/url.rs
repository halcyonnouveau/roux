/// Builds a url for read only Reddit access.
pub fn build_url(dest: &str) -> String {
    format!("https://www.reddit.com/{}/.json", dest)
}

/// Builds a url for OAuth Reddit access.
pub fn build_oauth(dest: &str) -> String {
    format!("https://oauth.reddit.com/{}/.json", dest)
}
