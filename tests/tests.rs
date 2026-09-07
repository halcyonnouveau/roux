extern crate dotenv;
extern crate roux;

#[cfg(all(not(feature = "blocking"), test))]
extern crate tokio;

#[cfg(test)]
mod tests {
    use std::env;

    use roux::saved::SavedData;
    use roux::util::FeedOption;
    use roux::Reddit;
    #[cfg(not(feature = "blocking"))]
    use tokio;

    #[allow(dead_code)]
    static USER_AGENT: &str = "macos:roux:v1.4.0 (by /u/beanpup_py)";

    /// Returns the app credentials, or `None` if either is missing or empty.
    #[allow(dead_code)]
    fn app_credentials() -> Option<(String, String)> {
        let client_id = env::var("CLIENT_ID").ok().filter(|v| !v.is_empty())?;
        let client_secret = env::var("CLIENT_SECRET").ok().filter(|v| !v.is_empty())?;
        Some((client_id, client_secret))
    }

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

        let new_client = Reddit::new(&USER_AGENT, &client_id, &client_secret)
            .username(&username)
            .password(&password)
            .subreddit("astolfo")
            .await
            .unwrap();

        assert!(new_client.top(10, None).await.is_ok());
        assert!(new_client.moderators().await.is_ok());
    }

    #[maybe_async::async_impl]
    #[tokio::test]
    async fn test_app_only() {
        dotenv::dotenv().ok();

        // Secrets are absent on forks and dependabot PRs, where GitHub sets
        // them to empty strings, so skip rather than fail there.
        let Some((client_id, client_secret)) = app_credentials() else {
            eprintln!("skipping test_app_only: CLIENT_ID and CLIENT_SECRET not set");
            return;
        };

        // No username or password: the client_credentials grant is used.
        let subreddit = Reddit::new(&USER_AGENT, &client_id, &client_secret)
            .subreddit("rust")
            .await
            .unwrap();

        let hot = subreddit.hot(5, None).await.unwrap();
        assert_eq!(hot.data.children.len(), 5);
        assert!(subreddit.about().await.is_ok());
        assert!(subreddit.latest_comments(None, Some(5)).await.is_ok());

        let article_id = hot.data.children.first().unwrap().data.id.clone();
        assert!(subreddit
            .article_comments(&article_id, None, Some(5))
            .await
            .is_ok());

        let user = Reddit::new(&USER_AGENT, &client_id, &client_secret)
            .user("spez")
            .await
            .unwrap();

        assert!(user.about(None).await.is_ok());
        let comments = user
            .comments(Some(FeedOption::new().limit(5)))
            .await
            .unwrap();
        assert!(!comments.data.children.is_empty());

        let subreddits = Reddit::new(&USER_AGENT, &client_id, &client_secret)
            .search_subreddits("rust", Some(3), None)
            .await
            .unwrap();
        assert_eq!(subreddits.data.children.len(), 3);

        // login() must keep requiring user credentials.
        let login = Reddit::new(&USER_AGENT, &client_id, &client_secret)
            .login()
            .await;
        assert!(matches!(
            login,
            Err(roux::util::RouxError::CredentialsNotSet)
        ));
    }

    #[allow(dead_code)]
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

        let new_client = Reddit::new(&USER_AGENT, &client_id, &client_secret)
            .username(&username)
            .password(&password)
            .subreddit("astolfo")
            .unwrap();

        assert!(new_client.top(10, None).is_ok());
        assert!(new_client.moderators().is_ok());
    }

    #[maybe_async::sync_impl]
    #[test]
    fn test_app_only() {
        dotenv::dotenv().ok();

        // Secrets are absent on forks and dependabot PRs, where GitHub sets
        // them to empty strings, so skip rather than fail there.
        let Some((client_id, client_secret)) = app_credentials() else {
            eprintln!("skipping test_app_only: CLIENT_ID and CLIENT_SECRET not set");
            return;
        };

        // No username or password: the client_credentials grant is used.
        let subreddit = Reddit::new(&USER_AGENT, &client_id, &client_secret)
            .subreddit("rust")
            .unwrap();

        let hot = subreddit.hot(5, None).unwrap();
        assert_eq!(hot.data.children.len(), 5);
        assert!(subreddit.about().is_ok());
        assert!(subreddit.latest_comments(None, Some(5)).is_ok());

        let article_id = hot.data.children.first().unwrap().data.id.clone();
        assert!(subreddit
            .article_comments(&article_id, None, Some(5))
            .is_ok());

        let user = Reddit::new(&USER_AGENT, &client_id, &client_secret)
            .user("spez")
            .unwrap();

        assert!(user.about(None).is_ok());
        let comments = user.comments(Some(FeedOption::new().limit(5))).unwrap();
        assert!(!comments.data.children.is_empty());

        let subreddits = Reddit::new(&USER_AGENT, &client_id, &client_secret)
            .search_subreddits("rust", Some(3), None)
            .unwrap();
        assert_eq!(subreddits.data.children.len(), 3);

        // login() must keep requiring user credentials.
        let login = Reddit::new(&USER_AGENT, &client_id, &client_secret).login();
        assert!(matches!(
            login,
            Err(roux::util::RouxError::CredentialsNotSet)
        ));
    }
}
