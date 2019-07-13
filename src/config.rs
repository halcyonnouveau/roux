pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub access_token: Option<String>,
}

impl Config {
    pub fn new(client_id: &str, client_secret: &str) -> Config {
        Config {
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            username: None,
            password: None,
            access_token: None,
        }
    }
}
