mod language;
mod selected_language;

pub use konnektoren_rs::platform::i18n::I18nConfig;
pub use language::{
    LANGUAGES, flag, language_name, log_language_info, normalize_language_code, supported_language,
    supported_language_code, supported_language_from_candidates,
};
pub use selected_language::SelectedLanguage;

pub const LANGUAGE_KEY: &str = "selected_language";
