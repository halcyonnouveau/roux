pub fn build_url(dest: &str) -> String {
    format!("https://www.reddit.com/{}/.json", dest)
}

pub fn build_oauth(dest: &str) -> String {
    format!("https://oauth.reddit.com/{}/.json", dest)
}
