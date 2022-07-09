#[derive(Debug, Clone)]
pub struct Config {
    pub user_agent: String,
    pub client_id: String,
    pub client_secret: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub access_token: Option<String>,
}

impl Config {
    pub fn new(user_agent: &str, client_id: &str, client_secret: &str) -> Config {
        Config {
            user_agent: user_agent.to_owned(),
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            username: None,
            password: None,
            access_token: None,
        }
    }
}
