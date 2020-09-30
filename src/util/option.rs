//! # Feed options
//! Listings do not use page numbers because their content changes so frequently.
//! Instead, they allow you to view slices of the underlying data.
//! Listing JSON responses contain after and before fields which are equivalent to the
//! "next" and "prev" buttons on the site and in combination with count can be used to page
//! through the listing.

/// Basic feed options
pub struct FeedOption {
    /// `after` and `before` indicate the fullname of an item in the listing to use as the anchor point of the slice.
    pub after: Option<String>,
    /// Only one should be specified.
    pub before: Option<String>,
    /// The number of items already seen in this listing.
    pub count: Option<u32>,
}

impl FeedOption {
    /// Create a new `FeedOption` instance.
    pub fn new() -> FeedOption {
        FeedOption {
            after: None,
            before: None,
            count: None,
        }
    }

    /// Set after param.
    pub fn after(mut self, ty: &str) -> FeedOption {
        if !self.before.is_none() {
            panic!("Cannot have an after and before param at the same time");
        }

        self.after = Some(ty.to_owned());
        self
    }

    /// Set before param.
    pub fn before(mut self, ty: &str) -> FeedOption {
        if !self.after.is_none() {
            panic!("Cannot have an after and before param at the same time");
        }

        self.before = Some(ty.to_owned());
        self
    }

    /// Set count param.
    pub fn count(mut self, ty: u32) -> FeedOption {
        self.count = Some(ty);
        self
    }

    /// build a url from FeedOption
    pub fn build_url(self, url: &mut String) {
        if let Some(after) = self.after {
            url.push_str(&mut format!("&after={}", after));
        } else if let Some(before) = self.before {
            url.push_str(&mut format!("&before={}", before));
        }

        if let Some(count) = self.count {
            url.push_str(&mut format!("&count={}", count));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FeedOption;

    #[test]
    fn test_build_url_after() {
        let after = "some_after";
        let options = FeedOption::new().after(after);

        let url = &mut String::from("");
        options.build_url(url);

        assert!(*url == format!("&after={}", after))
    }

    #[test]
    fn test_build_url_before() {
        let before = "some_before";
        let options = FeedOption::new().before(before);

        let url = &mut String::from("");
        options.build_url(url);

        assert!(*url == format!("&before={}", before))
    }

    #[test]
    fn test_build_url_count() {
        let count = 100u32;
        let options = FeedOption::new().count(count);

        let url = &mut String::from("");
        options.build_url(url);

        assert!(*url == format!("&count={}", count))
    }

    #[test]
    #[should_panic(expected = "Cannot have an after and before param at the same time")]
    fn test_feed_option_after_and_before() {
        FeedOption::new().after("after").before("before");
    }

    #[test]
    #[should_panic(expected = "Cannot have an after and before param at the same time")]
    fn test_feed_option_before_and_after() {
        FeedOption::new().before("before").after("after");
    }
}
