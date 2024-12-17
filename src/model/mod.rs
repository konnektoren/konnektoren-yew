//! This module contains the data model of the application.
mod design;
mod inbox;
mod session_initializer;
mod settings;

pub use design::Design;
pub use inbox::Inbox;
pub use session_initializer::{DefaultSessionInitializer, SessionInitializer};
pub use settings::Settings;
