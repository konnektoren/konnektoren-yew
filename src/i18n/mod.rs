//! The `i18n` module contains the internationalization (i18n) functionality for the application.

/// The `config` module contains the configuration for internationalization (i18n).
mod config;

/// The `provider` module defines the components and hooks for managing i18n within the application.
mod provider;

/// The `selected_language` module manages the state of the currently selected language.
pub use config::{I18nConfig, create_i18n_config};

pub use konnektoren_yew_i18n::{
    I18nJsonLoader, I18nLoader, I18nYmlLoader, flag, language_name, log_language_info,
    normalize_language_code, supported_language, supported_language_code,
    supported_language_from_candidates,
};

/// A constant key used to store the selected language in storage.
pub use konnektoren_yew_i18n::LANGUAGE_KEY;

/// A list of supported languages represented by their ISO codes.
pub use konnektoren_yew_i18n::LANGUAGES;

/// Re-export the i18n hooks and components from the `provider` module.
///
/// - `use_i18n`: A hook to access i18n functionality.
/// - `use_selected_language`: A hook to get or set the selected language.
/// - `I18nProvider`: A component that provides i18n context to the application.
/// - `I18nProviderProps`: The properties for the `I18nProvider` component.
pub use provider::{BrowserI18nProvider, I18nProvider, I18nProviderProps};

/// Re-export the `SelectedLanguage` type from the `selected_language` module.
pub use konnektoren_yew_i18n::{I18nContext, SelectedLanguage, use_i18n, use_selected_language};
