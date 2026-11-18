//! The review client of konnektoren-yew: HTTP with the app's trace and session headers.

#[cfg(feature = "csr")]
use konnektoren_yew_review::ReviewError;
use konnektoren_yew_review::{
    Review, ReviewClient, ReviewClientHandle, ReviewEndpoint, ReviewFuture,
};

/// Sends reviews to the Konnektoren API with traced requests.
pub(crate) struct TracedReviewClient {
    #[cfg_attr(not(feature = "csr"), allow(dead_code))]
    endpoint: ReviewEndpoint,
}

impl TracedReviewClient {
    pub(crate) fn handle(api_url: &str, default_endpoint: bool) -> ReviewClientHandle {
        ReviewClientHandle::new(Self {
            endpoint: ReviewEndpoint::with_default_endpoint(api_url, default_endpoint),
        })
    }
}

#[cfg(feature = "csr")]
impl ReviewClient for TracedReviewClient {
    fn submit(&self, review: Review) -> ReviewFuture<()> {
        use crate::tools::{TracedRequest, update_trace_from_response};
        use gloo::net::http::Request;

        let url = self.endpoint.submit_url();
        Box::pin(async move {
            let request = Request::post(&url)
                .with_trace()
                .json(&review)
                .map_err(|_| ReviewError::Network)?;
            let response = request.send().await.map_err(|error| {
                tracing::error!("Error while submitting review: {error:?}");
                ReviewError::Network
            })?;
            update_trace_from_response(&response);
            match response.status() {
                200..=299 => Ok(()),
                status => Err(ReviewError::Status(status)),
            }
        })
    }

    fn average_rating(&self, challenge_id: String) -> ReviewFuture<f64> {
        use crate::tools::TracedResponse;
        use gloo::net::http::Request;

        let url = self.endpoint.average_url(&challenge_id);
        Box::pin(async move {
            let response = Request::get(&url)
                .send_traced()
                .await
                .map_err(|_| ReviewError::Network)?;
            if response.status() != 200 {
                return Err(ReviewError::Status(response.status()));
            }
            response
                .json::<f64>()
                .await
                .map_err(|_| ReviewError::Network)
        })
    }
}

/// Without a browser, reviews can neither be sent nor loaded.
#[cfg(not(feature = "csr"))]
impl ReviewClient for TracedReviewClient {
    fn submit(&self, _: Review) -> ReviewFuture<()> {
        Box::pin(std::future::ready(Err(
            konnektoren_yew_review::ReviewError::Unavailable,
        )))
    }

    fn average_rating(&self, _: String) -> ReviewFuture<f64> {
        Box::pin(std::future::ready(Err(
            konnektoren_yew_review::ReviewError::Unavailable,
        )))
    }
}
