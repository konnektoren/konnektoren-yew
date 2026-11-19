//! Credits from an asset source: YAML first, license files only when opened.
mod catalog;
mod component;
mod group;
mod license_file;
mod license_text;
#[cfg(feature = "yew-preview")]
pub(crate) mod preview;

pub use component::{LicenseCredits, LicenseCreditsProps};
