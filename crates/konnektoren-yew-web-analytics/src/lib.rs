//! Backend-independent web analytics events with an optional GTM backend.
//!
//! Enable `gtm` and `csr` for Google Tag Manager browser delivery.
//!
//! ```ignore
//! use konnektoren_yew_web_analytics::{GtmAnalytics, Parameters, WebAnalytics};
//! use serde_json::json;
//!
//! let mut parameters = Parameters::new();
//! parameters.insert("item_id".into(), json!("learn"));
//! let _ = GtmAnalytics.track_event("music_navigation_favourite_added", parameters);
//! ```

mod event;
mod provider;
#[cfg(feature = "gtm")]
pub mod gtm;

pub use event::{Parameters, TrackError, TrackResult, WebAnalytics};
pub use provider::{AnalyticsContext, WebAnalyticsProvider, WebAnalyticsProviderProps, use_web_analytics};
#[cfg(feature = "gtm")]
pub use gtm::{GtmAnalytics, track_event};
