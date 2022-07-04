extern crate dotenv;
extern crate roux;

#[cfg(all(feature = "async", test))]
extern crate tokio;

#[cfg(test)]
mod tests {
    use std::env;

    use roux::me::responses::SavedData;
    use roux::util::FeedOption;
    use roux::Reddit;
    #[cfg(feature = "async")]
    use tokio;

    static USER_AGENT: &str = "macos:roux:v1.4.0 (by /u/beanpup_py)";

    #[maybe_async::async_impl]
    #[tokio::test]
    async fn test_oauth() {
        dotenv::dotenv().ok();

        let client_id = env::var("CLIENT_ID").unwrap();
        let client_secret = env::var("CLIENT_SECRET").unwrap();
        let username = env::var("USERNAME").unwrap();
        let password = env::var("PASSWORD").unwrap();

        let client = Reddit::new(&USER_AGENT, &client_id, &client_secret)
            .username(&username)
            .password(&password)
            .login()
            .await;

            assert!(client.is_ok());

            let me = client.unwrap();
    
            assert!(me.me().await.is_ok());
    
            let options = FeedOption::new().limit(5);
    
            // Assert FeedOption works
            let saved1 = me.saved(None).await.unwrap();
            let last_child_id1 = match &saved1.data.children.last().unwrap().data {
                SavedData::Comment(comments_data) => comments_data.id.as_ref().unwrap(),
                SavedData::Submission(submissions_data) => &submissions_data.id,
            };
    
            let saved2 = me
                .saved(Some(options.after(&saved1.data.after.unwrap())))
                .await
                .unwrap();
            let last_child_id2 = match &saved2.data.children.last().unwrap().data {
                SavedData::Comment(comments_data) => comments_data.id.as_ref().unwrap(),
                SavedData::Submission(submissions_data) => &submissions_data.id,
            };
    
            assert_ne!(last_child_id1, last_child_id2);
            assert_eq!(saved2.data.children.len(), 5);
    }

    #[maybe_async::sync_impl]
    fn test_oauth() {
        dotenv::dotenv().ok();

        let client_id = env::var("CLIENT_ID").unwrap();
        let client_secret = env::var("CLIENT_SECRET").unwrap();
        let username = env::var("USERNAME").unwrap();
        let password = env::var("PASSWORD").unwrap();
        
        let client = Reddit::new(&USER_AGENT, &client_id, &client_secret)
            .username(&username)
            .password(&password)
            .login();

            assert!(client.is_ok());

            let me = client.unwrap();
    
            assert!(me.me().is_ok());
    
            let options = FeedOption::new().limit(5);
    
            // Assert FeedOption works
            let saved1 = me.saved(None).unwrap();
            let last_child_id1 = match &saved1.data.children.last().unwrap().data {
                SavedData::Comment(comments_data) => comments_data.id.as_ref().unwrap(),
                SavedData::Submission(submissions_data) => &submissions_data.id,
            };
    
            let saved2 = me
                .saved(Some(options.after(&saved1.data.after.unwrap())))
                .unwrap();
            let last_child_id2 = match &saved2.data.children.last().unwrap().data {
                SavedData::Comment(comments_data) => comments_data.id.as_ref().unwrap(),
                SavedData::Submission(submissions_data) => &submissions_data.id,
            };
    
            assert_ne!(last_child_id1, last_child_id2);
            assert_eq!(saved2.data.children.len(), 5);
    }
}
