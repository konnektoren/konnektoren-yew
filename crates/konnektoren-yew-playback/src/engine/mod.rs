mod extensions;
mod playback;
#[cfg(feature = "yew-preview")]
mod preview;
mod target;
mod timer;

pub use extensions::{PlaybackExtensions, PlaybackWithExtensions};
pub use playback::{PlaybackEngine, PlaybackHost, PlaybackOutcome, PlaybackState, PlaybackToken};
#[cfg(feature = "yew-preview")]
pub use preview::PlaybackEnginePreview;
pub use target::use_playback_target;
pub use timer::use_playback_timer;
#[cfg(feature = "csr")]
mod browser;
#[cfg(feature = "csr")]
pub use browser::{
    BrowserTargetError, click_browser_target, fill_browser_target, resolve_browser_target,
    send_browser_key,
};
