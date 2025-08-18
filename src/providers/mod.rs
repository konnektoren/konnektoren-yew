//! # Providers
//!
//! This module contains Yew's `ContextProvider` components and associated hooks.
//! Providers are used to manage and share application-wide state and dependencies
//! (like repositories, game controllers, and user settings) across the component tree.
//!
//! They encapsulate state management logic and make data accessible to any descendant
//! component without prop drilling. This module also defines the `RepositoryConfig`
//! for setting up the application's persistence layer.
//!
//! The `use_*` hooks provide a convenient way to access the provided contexts.

pub mod certificates_provider;
pub mod client_side_router;
pub mod design_provider;
pub mod game_controller_provider;
pub mod inbox_provider;
pub mod profile_provider;
mod repository_context;
pub mod repository_hooks;
pub mod repository_provider;
pub mod session_provider;
pub mod settings_provider;
#[cfg(feature = "ssr")]
pub mod ssr_impl;
pub mod theme_provider;

pub use crate::i18n::{I18nProvider, I18nProviderProps, use_i18n, use_selected_language};
pub use certificates_provider::{CertificatesContext, CertificatesProvider};
pub use client_side_router::ClientSideRouter;
pub use design_provider::{DesignContext, DesignProvider, use_design};
pub use game_controller_provider::{
    GameControllerContext, GameControllerProvider, GameControllerProviderProps, use_command_bus,
    use_event_bus, use_game_controller, use_game_state,
};
pub use inbox_provider::{InboxContext, InboxProvider};
use profile_provider::{ProfileContext, ProfileProvider};
pub use repository_context::RepositoryContext;
pub use repository_hooks::{
    use_certificate_repository, use_certificates, use_inbox, use_inbox_repository, use_profile,
    use_profile_repository, use_session, use_session_repository, use_settings,
    use_settings_repository,
};
pub use repository_provider::{
    RepositoryConfig, RepositoryProvider, RepositoryProviderProps, create_repositories,
};
pub use session_provider::{SessionContext, SessionProvider};
pub use settings_provider::{SettingsContext, SettingsProvider};
pub use theme_provider::{ThemeContext, ThemeProvider, use_theme};
