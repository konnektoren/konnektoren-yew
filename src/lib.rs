//! # Konnektoren-Yew
//!
//! Konnektoren-Yew is the interactive web frontend for the Konnektoren language learning platform,
//! built with Rust and the Yew framework. This crate provides all the user interface components,
//! state management, and client-side logic necessary for the web application.
//!
//! It supports both Client-Side Rendering (CSR) for dynamic interactions and
//! Server-Side Rendering (SSR) for improved initial load performance and SEO.
//! The project is highly modular, leveraging Rust's powerful type system and
//! Yew's component-based architecture to create a maintainable and scalable application.
//!
//! ## Features
//!
//! -   **Interactive Challenges**: Various challenge types (multiple choice, gap fill, ordering).
//! -   **User Profiles & Progress**: Persistence of player data, XP, and achievements.
//! -   **Internationalization**: Comprehensive support for multiple languages.
//! -   **Theming & Design**: Customizable UI themes and responsive design modes.
//! -   **Web3 Wallet Integration**: Optional integration with blockchain wallets for marketplace features.
//! -   **Build-time SBOM**: Generates a Software Bill of Materials for enhanced supply chain security.
//!
//! For more details on running the project or contributing, please see the `README.md`.
//!
pub mod app;
pub mod app_ssr;
pub mod components;
pub mod i18n;
pub mod managers;
pub mod model;
pub mod providers;
pub mod tools;

#[cfg(feature = "effects")]
pub mod effects;

#[cfg(feature = "storage")]
pub mod repository;

/// This is a prelude module that re-exports the most important types and traits.
pub mod prelude {
    #[cfg(feature = "csr")]
    pub use crate::app::App;
    #[cfg(feature = "ssr")]
    pub use crate::app_ssr::App;
    pub use crate::components::*;
    #[cfg(feature = "effects")]
    pub use crate::effects::*;
    pub use crate::i18n::*;
    pub use crate::managers::*;
    pub use crate::model::*;
    pub use crate::providers::*;
    pub use crate::tools::{TracedRequest, TracedResponse, update_trace_from_response};
}
