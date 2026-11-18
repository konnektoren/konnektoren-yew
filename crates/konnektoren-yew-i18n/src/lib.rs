//! Shared internationalization context and hooks.
pub mod loader;
pub mod model;
pub mod provider;

pub use loader::{I18nJsonLoader, I18nLoader, I18nYmlLoader};
pub use model::{
    I18nConfig, LANGUAGE_KEY, LANGUAGES, SelectedLanguage, flag, language_name, log_language_info,
    normalize_language_code, supported_language, supported_language_code,
    supported_language_from_candidates,
};
pub use provider::{I18nContext, use_i18n, use_selected_language};
