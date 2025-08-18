//! # Data Models
//!
//! This module contains the core data structures and models used throughout the Konnektoren-Yew application.
//! These models represent the application's state, user preferences, and domain-specific entities,
//! designed to be serialized and deserialized for persistence and communication.
//!
//! Key models include:
//! - [`Design`]: Defines the visual design modes (e.g., desktop, mobile).
//! - [`Inbox`]: Represents the user's message inbox.
//! - [`SessionInitializer`]: A trait for initializing user sessions.
//! - [`Settings`]: Stores user-configurable application settings.
//! - [`Theme`]: Defines the UI themes (e.g., light, dark).
//!
//! These models are central to the application's data flow and state management.

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
