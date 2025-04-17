//! The `i18n` module contains the internationalization (i18n) functionality for the application.

/// The `config` module contains the configuration for internationalization (i18n).
mod config;

/// The `provider` module defines the components and hooks for managing i18n within the application.
mod provider;

mod i18n_json_loader;
mod i18n_loader;
mod i18n_yml_loader;
/// The `selected_language` module manages the state of the currently selected language.
mod selected_language;

pub use config::{create_i18n_config, I18nConfig};

pub use i18n_json_loader::I18nJsonLoader;
pub use i18n_loader::I18nLoader;
pub use i18n_yml_loader::I18nYmlLoader;

/// A constant key used to store the selected language in storage.
pub const LANGUAGE_KEY: &str = "selected_language";

/// A list of supported languages represented by their ISO codes.
pub const LANGUAGES: [&str; 9] = ["en", "uk", "ar", "de", "zh", "pl", "tr", "es", "vi"];

/// Re-export the i18n hooks and components from the `provider` module.
///
/// - `use_i18n`: A hook to access i18n functionality.
/// - `use_selected_language`: A hook to get or set the selected language.
/// - `I18nProvider`: A component that provides i18n context to the application.
/// - `I18nProviderProps`: The properties for the `I18nProvider` component.
pub use provider::{use_i18n, use_selected_language, I18nProvider, I18nProviderProps};

/// Re-export the `SelectedLanguage` type from the `selected_language` module.
pub use selected_language::SelectedLanguage;

/// Checks if the provided language is supported by the application.
///
/// # Parameters
///
/// - `lang`: An optional string slice representing the language code.
///
/// # Returns
///
/// If the language is supported, it returns `Some(String)` with the language code as a `String`.
/// Otherwise, it returns `None`.
///
/// # Examples
///
/// ```
/// use konnektoren_yew::i18n::supported_language;
/// let lang = supported_language(Some("en"));
/// assert_eq!(lang, Some("en".to_string()));
/// ```
pub fn supported_language(lang: Option<&str>) -> Option<String> {
    match lang {
        Some(lang) => {
            if LANGUAGES.contains(&lang) {
                Some(lang.to_string())
            } else {
                None
            }
        }
        None => None,
    }
}

/// Returns the flag emoji corresponding to the provided language code.
///
/// # Parameters
///
/// - `lang`: A static string slice representing the language code.
///
/// # Returns
///
/// A static string slice containing the flag emoji associated with the language code.
///
/// # Examples
///
/// ```
/// use konnektoren_yew::i18n::flag;
/// let flag = flag("en");
/// assert_eq!(flag, "🇺🇸");
/// ```
pub fn flag(lang: &'static str) -> &'static str {
    match lang {
        "en" => "🇺🇸",
        "de" => "🇩🇪",
        "uk" => "🇺🇦",
        "zh" => "🇨🇳",
        "ar" => "🇸🇦",
        "pl" => "🇵🇱",
        "tr" => "🇹🇷",
        "es" => "🇪🇸",
        "vi" => "🇻🇳",
        _ => "🌐",
    }
}

pub fn language_name(lang: &'static str) -> &'static str {
    match lang {
        "en" => "English",
        "de" => "Deutsch",
        "uk" => "Українська",
        "zh" => "中文",
        "ar" => "العربية",
        "pl" => "Polski",
        "tr" => "Türkçe",
        "es" => "Español",
        "vi" => "Tiếng Việt",
        _ => "🌐",
    }
}

pub fn log_language_info(context: &str) {
    #[cfg(feature = "ssr")]
    {
        use log::{info, warn};

        // Get language from environment and log it along with the flag
        if let Ok(lang) = std::env::var("LANG") {
            let flag_emoji = match lang.as_str() {
                "en" => "🇺🇸",
                "de" => "🇩🇪",
                "uk" => "🇺🇦",
                "zh" => "🇨🇳",
                "ar" => "🇸🇦",
                "pl" => "🇵🇱",
                "tr" => "🇹🇷",
                "es" => "🇪🇸",
                "vi" => "🇻🇳",
                _ => "🌐",
            };

            let lang_name = match lang.as_str() {
                "en" => "English",
                "de" => "Deutsch",
                "uk" => "Українська",
                "zh" => "中文",
                "ar" => "العربية",
                "pl" => "Polski",
                "tr" => "Türkçe",
                "es" => "Español",
                "vi" => "Tiếng Việt",
                _ => "Unknown",
            };

            info!(
                "🌐 LANG={}  {} {} | Context: {}",
                lang, flag_emoji, lang_name, context
            );
        } else {
            warn!(
                "⚠️ No LANG environment variable set! | Context: {}",
                context
            );
        }

        // Also check all environment variables related to language
        for (key, value) in std::env::vars() {
            if key.contains("LANG") || key.contains("lang") || key.contains("LOCALE") {
                info!("  ENV: {}={}", key, value);
            }
        }
    }
}

/// Unit tests for the `supported_language` and `flag` functions.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests the `supported_language` function with various inputs.
    fn test_supported_language() {
        assert_eq!(supported_language(Some("en")), Some("en".to_string()));
        assert_eq!(supported_language(Some("uk")), Some("uk".to_string()));
        assert_eq!(supported_language(Some("de")), Some("de".to_string()));
        assert_eq!(supported_language(Some("zh")), Some("zh".to_string()));
        assert_eq!(supported_language(Some("ar")), Some("ar".to_string()));
        assert_eq!(supported_language(Some("pl")), Some("pl".to_string()));
        assert_eq!(supported_language(Some("tr")), Some("tr".to_string()));
        assert_eq!(supported_language(Some("es")), Some("es".to_string()));
        assert_eq!(supported_language(Some("vi")), Some("vi".to_string()));
        assert_eq!(supported_language(Some("fr")), None);

        assert_eq!(supported_language(None), None);
    }

    #[test]
    /// Tests the `flag` function to ensure it returns the correct flag emoji.
    fn test_flag() {
        assert_eq!(flag("en"), "🇺🇸");
        assert_eq!(flag("de"), "🇩🇪");
        assert_eq!(flag("uk"), "🇺🇦");
        assert_eq!(flag("zh"), "🇨🇳");
        assert_eq!(flag("ar"), "🇸🇦");
        assert_eq!(flag("pl"), "🇵🇱");
        assert_eq!(flag("tr"), "🇹🇷");
        assert_eq!(flag("es"), "🇪🇸");
        assert_eq!(flag("vi"), "🇻🇳");
        assert_eq!(flag("fr"), "🌐");
    }

    #[test]
    /// Tests the `name` function to ensure it returns the correct language name.
    fn test_language_name() {
        assert_eq!(language_name("en"), "English");
        assert_eq!(language_name("de"), "Deutsch");
        assert_eq!(language_name("uk"), "Українська");
        assert_eq!(language_name("zh"), "中文");
        assert_eq!(language_name("ar"), "العربية");
        assert_eq!(language_name("pl"), "Polski");
        assert_eq!(language_name("tr"), "Türkçe");
        assert_eq!(language_name("es"), "Español");
        assert_eq!(language_name("vi"), "Tiếng Việt");
        assert_eq!(language_name("fr"), "🌐");
    }
}
