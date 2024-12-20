//! This module contains the data model of the application.
mod design;
mod inbox;
mod session_initializer;
mod settings;
mod theme;

pub use design::Design;
pub use inbox::Inbox;
pub use session_initializer::{DefaultSessionInitializer, SessionInitializer};
pub use settings::Settings;
pub use theme::Theme;
