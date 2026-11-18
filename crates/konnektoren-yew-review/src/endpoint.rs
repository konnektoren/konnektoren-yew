/// Where reviews are sent and average ratings are read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReviewEndpoint {
    api_url: String,
    default_endpoint: bool,
}

impl ReviewEndpoint {
    /// An API base URL such as `https://api.konnektoren.help/api/v1`.
    /// Reviews go to `{api_url}/reviews`.
    pub fn new(api_url: impl Into<String>) -> Self {
        Self {
            api_url: api_url.into(),
            default_endpoint: true,
        }
    }

    /// A URL used as it is, without appending the default review paths.
    pub fn raw(url: impl Into<String>) -> Self {
        Self {
            api_url: url.into(),
            default_endpoint: false,
        }
    }

    /// Choose between [`Self::new`] and [`Self::raw`].
    pub fn with_default_endpoint(api_url: impl Into<String>, default_endpoint: bool) -> Self {
        if default_endpoint {
            Self::new(api_url)
        } else {
            Self::raw(api_url)
        }
    }

    /// URL for `POST` of a review.
    pub fn submit_url(&self) -> String {
        if !self.default_endpoint {
            return self.api_url.clone();
        }
        format!("{}/reviews", self.base())
    }

    /// URL for `GET` of the average rating of a challenge.
    pub fn average_url(&self, challenge_id: &str) -> String {
        if !self.default_endpoint {
            return self.api_url.clone();
        }
        format!("{}/reviews/{}/average", self.base(), challenge_id)
    }

    fn base(&self) -> &str {
        self.api_url.trim_end_matches('/')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_endpoint_appends_review_paths() {
        let endpoint = ReviewEndpoint::new("https://api.example/api/v1/");
        assert_eq!(endpoint.submit_url(), "https://api.example/api/v1/reviews");
        assert_eq!(
            endpoint.average_url("music:c-major"),
            "https://api.example/api/v1/reviews/music:c-major/average"
        );
    }

    #[test]
    fn raw_endpoint_is_used_unchanged() {
        let endpoint = ReviewEndpoint::with_default_endpoint("https://api.example/custom", false);
        assert_eq!(endpoint.submit_url(), "https://api.example/custom");
        assert_eq!(endpoint.average_url("id"), "https://api.example/custom");
    }
}
