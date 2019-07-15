extern crate dotenv;
extern crate roux;

#[cfg(test)]
mod tests {
    use roux::Reddit;

    #[test]
    fn test_oauth_wrong() {
        let client_id = "XXXXXXXX";
        let client_secret = "XXXXXXXXXX";
        let username = "jim_pickens";
        let password = "password123";

        let client = Reddit::new(&client_id, &client_secret)
            .username(&username)
            .password(&password)
            .login();

        assert!(!client.is_ok());
    }

    #[test]
    fn test_oauth_correct() {
        let client_id = dotenv::var("CLIENT_ID").unwrap();
        let client_secret = dotenv::var("CLIENT_SECRET").unwrap();
        let username = dotenv::var("USERNAME").unwrap();
        let password = dotenv::var("PASSWORD").unwrap();

        let me_result = Reddit::new(&client_id, &client_secret)
            .username(&username)
            .password(&password)
            .login();

        assert!(me_result.is_ok());

        let me = me_result.unwrap();

        assert!(me.get().is_ok());
        assert!(me.logout().is_ok());
    }
}
