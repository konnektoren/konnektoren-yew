//! Completion presentation without routing or persistence. Apps supply translated
//! headings, actions and custom reviews; built-in result types share one dispatcher.
mod completion;
mod details;

pub use completion::{ChallengeCompletion, ChallengeCompletionProps};
pub use details::{ChallengeResultDetails, ChallengeResultDetailsProps};

#[cfg(feature = "yew-preview")]
mod preview;
