use crate::{Review, ReviewClient, ReviewClientHandle, ReviewError, ReviewFuture};

/// A client for previews: a fixed average, and a submit that succeeds or fails.
pub(crate) struct PreviewClient {
    pub average: Option<f64>,
    pub submit_error: Option<ReviewError>,
}

impl PreviewClient {
    pub(crate) fn handle(
        average: Option<f64>,
        submit_error: Option<ReviewError>,
    ) -> ReviewClientHandle {
        ReviewClientHandle::new(Self {
            average,
            submit_error,
        })
    }
}

impl ReviewClient for PreviewClient {
    fn submit(&self, _: Review) -> ReviewFuture<()> {
        let result = self.submit_error.clone().map_or(Ok(()), Err);
        Box::pin(std::future::ready(result))
    }

    fn average_rating(&self, _: String) -> ReviewFuture<f64> {
        let result = self.average.ok_or(ReviewError::Network);
        Box::pin(std::future::ready(result))
    }
}
