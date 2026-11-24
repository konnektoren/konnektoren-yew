use chrono::{DateTime, Utc};
use yew::AttrValue;

/// A point in time to show: parsed, or a stored timestamp string.
#[derive(Clone, Debug, PartialEq)]
pub enum TimeValue {
    At(DateTime<Utc>),
    /// A stored timestamp string, shown as `YYYY-MM-DD HH:MM`.
    Raw(AttrValue),
}

impl From<DateTime<Utc>> for TimeValue {
    fn from(time: DateTime<Utc>) -> Self {
        Self::At(time)
    }
}
