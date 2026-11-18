//! Challenge reviews: star ratings, an optional comment and the average rating.
//!
//! The components do not know how reviews are sent. The host app provides a
//! [`ReviewClient`] (for example an HTTP client with its own tracing headers)
//! and the translated [`ReviewLabels`].
//!
//! With the `web-analytics` feature, the review form reports
//! [`REVIEW_SUBMITTED_EVENT`] and [`REVIEW_FAILED_EVENT`] to the
//! `konnektoren-yew-web-analytics` provider. Comment text is never tracked.
//!
//! Styles: `styles/crates/review/review.css` in the `konnektoren-yew`
//! repository (Tailwind + daisyUI). These are included by its `styles/index.css`.

mod client;
mod components;
mod endpoint;
mod labels;

#[cfg(feature = "web-analytics")]
mod analytics;

pub use client::{ReviewClient, ReviewClientHandle, ReviewError, ReviewFuture};
pub use components::{
    AverageRating, AverageRatingProps, RatingStarsComponent, RatingStarsProps, ReviewForm,
    ReviewFormProps, ReviewList, ReviewListProps,
};
pub use endpoint::ReviewEndpoint;
pub use konnektoren_core::challenges::Review;
pub use labels::ReviewLabels;

#[cfg(feature = "web-analytics")]
pub use analytics::{REVIEW_FAILED_EVENT, REVIEW_SUBMITTED_EVENT, review_parameters};
