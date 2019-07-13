extern crate dotenv;
extern crate roux;

#[cfg(test)]
mod tests {
    use roux::Reddit;

    #[test]
    fn test_oauth_wrong() {
        let client_id = "20000208";
        let client_secret = "randomletters";
        let username = "jim_pickens";
        let password = "password123";

        let client = Reddit::new(&client_id, &client_secret)
            .username(&username)
            .password(&password)
            .login();

        assert!(!client.is_ok());
    }

    fn test_oauth_correct() {
        let client_id = dotenv::var("CLIENT_ID").unwrap();
        let client_secret = dotenv::var("CLIENT_SECRET").unwrap();
        let username = dotenv::var("USERNAME").unwrap();
        let password = dotenv::var("PASSWORD").unwrap();

        let client = Reddit::new(&client_id, &client_secret)
            .username(&username)
            .password(&password)
            .login();

        // assert!(client.is_ok());
    }
}
