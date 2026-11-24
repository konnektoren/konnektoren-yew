mod keys;
mod playback;
#[cfg(feature = "yew-preview")]
mod preview;
mod runtime;
mod target;
mod timer;

pub use keys::use_playback_keys;
pub use playback::{PlaybackEngine, PlaybackHost, PlaybackOutcome, PlaybackState, PlaybackToken};
#[cfg(feature = "yew-preview")]
pub use preview::PlaybackEnginePreview;
pub use runtime::{
    ClickEffect, PlaybackCommand, PlaybackControls, PlaybackHandle, PlaybackOptions, StepRunner,
    StepTask, use_playback,
};
pub use target::{PlaybackTarget, use_playback_target};
pub use timer::use_playback_timer;
#[cfg(feature = "csr")]
mod browser;
#[cfg(feature = "csr")]
pub use browser::{
    BrowserTargetError, click_browser_target, fill_browser_element, fill_browser_target,
    resolve_browser_target, scroll_to_target, send_browser_key,
};
