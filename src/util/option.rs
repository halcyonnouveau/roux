//! # Feed options
//! Listings do not use page numbers because their content changes so frequently.
//! Instead, they allow you to view slices of the underlying data.
//! Listing JSON responses contain after and before fields which are equivalent to the
//! "next" and "prev" buttons on the site and in combination with count can be used to page
//! through the listing.

/// Basic feed options
#[derive(Clone, Debug)]
pub struct FeedOption {
    /// `after` and `before` indicate the fullname of an item in the listing to use as the anchor point of the slice.
    pub after: Option<String>,
    /// Only one should be specified.
    pub before: Option<String>,
    /// The number of items that can be in this listing.
    pub limit: Option<u32>,
    /// The number of items already seen in this listing.
    pub count: Option<u32>,
    /// What time period to request (only works on some requests, like top)
    pub period: Option<TimePeriod>,
}

impl FeedOption {
    /// Create a new `FeedOption` instance.
    pub fn new() -> FeedOption {
        FeedOption {
            after: None,
            before: None,
            count: None,
            limit: None,
            period: None,
        }
    }

    /// Set after param.
    pub fn after(mut self, ty: &str) -> FeedOption {
        if self.before.is_some() {
            panic!("Cannot have an after and before param at the same time");
        }

        self.after = Some(ty.to_owned());
        self
    }

    /// Set before param.
    pub fn before(mut self, ty: &str) -> FeedOption {
        if self.after.is_some() {
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

    /// Set limit param.
    pub fn limit(mut self, ty: u32) -> FeedOption {
        self.limit = Some(ty);
        self
    }

    /// Set period
    pub fn period(mut self, period: TimePeriod) -> FeedOption {
        self.period = Some(period);
        self
    }

    /// Build a url from `FeedOption`
    pub fn build_url(self, url: &mut String) {
        // Add a fake url attr so I don't have to parse things
        url.push_str(&String::from("?"));

        if let Some(after) = self.after {
            url.push_str(&format!("&after={}", after));
        } else if let Some(before) = self.before {
            url.push_str(&format!("&before={}", before));
        }

        if let Some(count) = self.count {
            url.push_str(&format!("&count={}", count));
        }

        if let Some(limit) = self.limit {
            url.push_str(&format!("&limit={}", limit));
        }

        if let Some(period) = self.period {
            url.push_str(&format!("&t={}", period.get_string_for_period()));
        }

        // HACK : the previous option won't work if a '&' isn't appended for some reason
        // Eg. &after={} won't return correct page
        // Eg. &after={}&limit={} returns correct page but won't return correct limit
        // I have no idea why.
        url.push_str(&String::from("&"));
    }
}

impl Default for FeedOption {
    fn default() -> Self {
        Self::new()
    }
}

/// Allows you to request a certain time period. This only works in certain situations, like when asking for top of a subreddit
#[derive(Copy, Clone, Debug)]
pub enum TimePeriod {
    /// Posts from very recently
    Now,
    /// Posts from today
    Today,
    /// Posts from this week
    ThisWeek,
    /// Posts from this month
    ThisMonth,
    /// Posts from this year
    ThisYear,
    /// All posts
    AllTime,
}

impl TimePeriod {
    /// Gets the request string for the period
    pub fn get_string_for_period(&self) -> &str {
        match self {
            TimePeriod::Now => "now",
            TimePeriod::Today => "day",
            TimePeriod::ThisWeek => "week",
            TimePeriod::ThisMonth => "month",
            TimePeriod::ThisYear => "year",
            TimePeriod::AllTime => "all",
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

        assert!(*url == format!("?&after={}&", after))
    }

    #[test]
    fn test_build_url_before() {
        let before = "some_before";
        let options = FeedOption::new().before(before);

        let url = &mut String::from("");
        options.build_url(url);

        assert!(*url == format!("?&before={}&", before))
    }

    #[test]
    fn test_build_url_count() {
        let count = 100u32;
        let options = FeedOption::new().count(count);

        let url = &mut String::from("");
        options.build_url(url);

        assert!(*url == format!("?&count={}&", count))
    }
}
