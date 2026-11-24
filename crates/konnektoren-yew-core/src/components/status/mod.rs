//! Reusable resource-state presentation. Messages and titles are already translated;
//! consumers own fetching, retry logic, and the transition back to content.
mod empty;
mod error;
mod loading;
mod message;

pub use empty::{EmptyState, EmptyStateProps};
pub use error::{ErrorState, ErrorStateProps};
pub use loading::{LoadingState, LoadingStateProps};
pub use message::{StatusMessage, StatusMessageProps};
// The data types live in `model`; kept here so `status::StatusType` still resolves.
pub use crate::model::{LoadingIndicator, StatusType};
