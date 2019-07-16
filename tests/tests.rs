extern crate dotenv;
extern crate roux;

#[cfg(test)]
mod tests {
    use roux::Reddit;

    static USER_AGENT: &str = "macos:roux:v0.1.2 (by /u/beanpup_py)";

    #[test]
    fn test_oauth() {
        let client_id = dotenv::var("CLIENT_ID").unwrap();
        let client_secret = dotenv::var("CLIENT_SECRET").unwrap();
        let username = dotenv::var("USERNAME").unwrap();
        let password = dotenv::var("PASSWORD").unwrap();

        let client = Reddit::new(&USER_AGENT, &client_id, &client_secret)
            .username(&username)
            .password(&password)
            .login();

        assert!(client.is_ok());

        let me = client.unwrap();

        assert!(me.me().is_ok());
        assert!(me.logout().is_ok());
    }
}
