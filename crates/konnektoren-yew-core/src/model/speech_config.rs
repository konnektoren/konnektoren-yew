/// Display name for the browser's automatic language-specific voice selection.
pub const DEFAULT_VOICE_NAME: &str = "System default";

/// Settings for speech produced by descendants of a [`SpeechProvider`](crate::providers::SpeechProvider).
#[derive(Clone, Debug, PartialEq)]
pub struct SpeechConfig {
    /// Volume from 0 (muted) to 1 (full volume).
    pub volume: f32,
    /// Browser voice URI, or [`DEFAULT_VOICE_NAME`] for automatic selection.
    /// When omitted, unavailable, or incompatible with the
    /// utterance language, use the browser's language-specific default.
    pub voice: Option<String>,
}

impl Default for SpeechConfig {
    fn default() -> Self {
        Self {
            volume: 0.3,
            voice: Some(DEFAULT_VOICE_NAME.to_owned()),
        }
    }
}
