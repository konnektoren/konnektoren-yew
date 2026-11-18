use crate::Review;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

/// A boxed, non-`Send` future, as used in the browser.
pub type ReviewFuture<T> = Pin<Box<dyn Future<Output = Result<T, ReviewError>>>>;

/// Why a review could not be sent or a rating could not be read.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ReviewError {
    /// The server answered with a non-success status.
    #[error("Failed to submit ({0})")]
    Status(u16),
    /// The server could not be reached.
    #[error("Could not reach server")]
    Network,
    /// Reviews cannot be sent in this build (for example during SSR).
    #[error("Reviews are unavailable")]
    Unavailable,
}

impl ReviewError {
    /// Short, stable reason for analytics, e.g. `status_500` or `network`.
    pub fn reason(&self) -> String {
        match self {
            Self::Status(status) => format!("status_{status}"),
            Self::Network => "network".into(),
            Self::Unavailable => "unavailable".into(),
        }
    }
}

/// Sends reviews and reads average ratings. Implemented by the host app.
pub trait ReviewClient {
    /// Store a review.
    fn submit(&self, review: Review) -> ReviewFuture<()>;
    /// The average rating of a challenge.
    fn average_rating(&self, challenge_id: String) -> ReviewFuture<f64>;
}

/// Shared review client, passed to the components as a prop.
#[derive(Clone)]
pub struct ReviewClientHandle(pub Rc<dyn ReviewClient>);

impl ReviewClientHandle {
    pub fn new(client: impl ReviewClient + 'static) -> Self {
        Self(Rc::new(client))
    }
}

impl PartialEq for ReviewClientHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl ReviewClient for ReviewClientHandle {
    fn submit(&self, review: Review) -> ReviewFuture<()> {
        self.0.submit(review)
    }

    fn average_rating(&self, challenge_id: String) -> ReviewFuture<f64> {
        self.0.average_rating(challenge_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_have_stable_analytics_reasons() {
        assert_eq!(ReviewError::Status(503).reason(), "status_503");
        assert_eq!(ReviewError::Network.reason(), "network");
        assert_eq!(ReviewError::Unavailable.reason(), "unavailable");
    }

    #[test]
    fn errors_keep_the_previous_messages() {
        assert_eq!(
            ReviewError::Status(400).to_string(),
            "Failed to submit (400)"
        );
        assert_eq!(ReviewError::Network.to_string(), "Could not reach server");
    }
}
