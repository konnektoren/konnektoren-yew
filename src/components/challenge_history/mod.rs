//! Presentational history views. Applications own persistence and result navigation.
mod attempts;
mod history;
mod list;
mod summary;
mod table;

#[cfg(feature = "yew-preview")]
mod preview;

pub use attempts::{ChallengeAttemptHistory, ChallengeAttemptHistoryProps};
pub use history::ChallengeHistoryProps;
pub use list::ChallengeHistoryList;
pub use summary::{ChallengeHistorySummaryComponent, ChallengeHistorySummaryProps};
pub use table::ChallengeHistoryTable;
