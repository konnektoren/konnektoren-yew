//! What the status panels show, without rendering.

/// Kind and text of a [`StatusMessage`](crate::StatusMessage).
#[derive(Clone, Debug, PartialEq)]
pub enum StatusType {
    Text(String),
    Info(String),
    Warning(String),
    Error(String),
}

impl Default for StatusType {
    fn default() -> Self {
        StatusType::Text("This is a text message".to_string())
    }
}

impl StatusType {
    /// The message, already translated.
    pub fn text(&self) -> &str {
        match self {
            Self::Text(text) | Self::Info(text) | Self::Warning(text) | Self::Error(text) => text,
        }
    }

    /// BEM modifier name: `text`, `info`, `warning`, `error`.
    pub fn modifier(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Info(_) => "info",
            Self::Warning(_) => "warning",
            Self::Error(_) => "error",
        }
    }

    /// ARIA role: errors are alerts, info and warnings polite status; plain text none.
    pub fn role(&self) -> Option<&'static str> {
        match self {
            Self::Error(_) => Some("alert"),
            Self::Info(_) | Self::Warning(_) => Some("status"),
            Self::Text(_) => None,
        }
    }
}

/// The placeholder a [`LoadingState`](crate::LoadingState) shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LoadingIndicator {
    #[default]
    Spinner,
    Skeleton,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_types_map_to_modifier_and_role() {
        let error = StatusType::Error("Oops".into());
        assert_eq!(
            (error.text(), error.modifier(), error.role()),
            ("Oops", "error", Some("alert"))
        );
        assert_eq!(StatusType::Text("Hi".into()).role(), None);
        assert_eq!(StatusType::Warning("Careful".into()).role(), Some("status"));
    }
}
