pub use konnektoren_platform::i18n::I18nConfig;
use konnektoren_platform::i18n::{CombinedTranslationAsset, Language, TranslationAsset};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/assets/i18n/"]
pub struct LocalI18nAssets;

pub fn create_i18n_config() -> I18nConfig {
    let mut config = I18nConfig::with_assets(CombinedTranslationAsset::<
        konnektoren_platform::i18n::I18nAssets,
    >::new("i18n.yml"));

    // Set default language based on environment variable in SSR mode
    #[cfg(feature = "ssr")]
    {
        if let Ok(lang_code) = std::env::var("LANG") {
            log::debug!(
                "🌐 Setting default language from LANG environment variable: {}",
                lang_code
            );
            config.default_language = Language::from_code(&lang_code);
        } else {
            log::warn!("⚠️ LANG environment variable not set in SSR mode, using default language");
        }
    }

    // Then merge local translations
    let local_translations =
        CombinedTranslationAsset::<LocalI18nAssets>::new("i18n.yml").load_translations();

    // Merge local translations into platform config
    for (lang_code, translations) in local_translations {
        if let Some(lang) = Language::builtin()
            .into_iter()
            .find(|l| l.code() == lang_code)
        {
            config.merge_translation(&lang, translations);
        }
    }
    config
}

#[cfg(test)]
mod tests {
    use super::*;
    use konnektoren_platform::prelude::Language;
    use serde_json::json;

    #[test]
    fn test_create_i18n() {
        let i18n = create_i18n_config();
        assert!(i18n.translations.contains_key("en"));
        assert!(i18n.translations.contains_key("de"));
        assert_eq!(i18n.default_language, Language::default());
    }

    #[test]
    fn test_translations() {
        let i18n = create_i18n_config();

        // Test JSON translations
        assert_eq!(i18n.t("Language"), "Language");
        assert_eq!(
            i18n.t_with_lang("Language", &Language::from("de")),
            "Sprache"
        );

        // Test YAML translations
        assert_eq!(i18n.t("Description"), "Description");
        assert_eq!(
            i18n.t_with_lang("Description", &Language::from("de")),
            "Beschreibung"
        );
    }

    #[test]
    fn test_supported_languages() {
        let i18n = create_i18n_config();
        let supported = i18n.supported_languages();

        // Check if all builtin languages are supported
        for lang in ["en", "de", "es", "ar", "zh", "uk", "pl", "tr", "vi"].iter() {
            assert!(
                supported.iter().any(|l| l.code() == *lang),
                "Language {} should be supported",
                lang
            );
        }
    }

    #[test]
    fn test_merge_translations_config() {
        // Test merging translations with I18nConfig
        let mut config = create_i18n_config();
        let new_trans = json!({
            "NewKey": "New Value",
            "AnotherKey": "Another Value"
        });

        let lang = Language::from("en");
        config.merge_translation(&lang, new_trans);

        assert_eq!(config.get_translation("NewKey", Some(&lang)), "New Value");
        assert_eq!(
            config.get_translation("AnotherKey", Some(&lang)),
            "Another Value"
        );

        // Test that original translations are preserved
        assert_eq!(config.get_translation("Language", Some(&lang)), "Language");
    }

    #[test]
    fn test_fallback_behavior() {
        let i18n = create_i18n_config();
        let missing_key = "NonExistentKey";

        // Should return the key itself when translation is missing
        assert_eq!(
            i18n.t_with_lang(missing_key, &Language::from("en")),
            missing_key
        );

        // Should fall back to default language for unsupported language
        assert_eq!(
            i18n.t_with_lang("Language", &Language::from("xx")),
            "Language"
        );
    }

    #[test]
    fn test_loaded_assets() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .is_test(true)
            .try_init();

        // First, let's check what files are actually embedded
        log::info!("Available asset files:");
        for file in LocalI18nAssets::iter() {
            log::info!("  {}", file);
        }

        let config = create_i18n_config();

        // Log raw translations map
        log::info!("Raw translations map:");
        log::info!("{:#?}", config.translations);

        // Try to load a specific file directly
        if let Some(content) = LocalI18nAssets::get("de.json") {
            let content_str = std::str::from_utf8(&content.data).unwrap();
            log::info!("Content of de.json:\n{}", content_str);
        } else {
            log::error!("Could not load de.json");
        }
        let config = create_i18n_config();

        // Test German translations
        let de_translations = config
            .translations
            .get("de")
            .expect("German translations should exist");
        let de_obj = de_translations.as_object().expect("Should be an object");

        // Test specific German translations
        assert_eq!(de_obj["Language"].as_str().unwrap(), "Sprache");
        assert_eq!(
            de_obj["Please select a language from the dropdown."]
                .as_str()
                .unwrap(),
            "Bitte wählen Sie eine Sprache aus dem Dropdown-Menü."
        );
        assert_eq!(de_obj["Tasks"].as_str().unwrap(), "Aufgaben");
        assert_eq!(
            de_obj["Unlock Points"].as_str().unwrap(),
            "Freizuschaltende Punkte"
        );

        // Test that all expected keys exist in German translations
        let expected_de_keys = vec![
            "Language",
            "Please select a language from the dropdown.",
            "Please select a language:",
            "Select Language",
            "Tasks",
            "Unlock Points",
            "Rate this Challenge",
            "Submit",
            "Submitting...",
            "Leave a comment",
            "Thank you for your review!",
            "Name",
            "Performance",
            "Time",
            "Rank",
            "Show Tour Button",
            "Start Tour",
            "Feedback",
            "We'd love your feedback!",
        ];

        for key in expected_de_keys {
            assert!(
                de_obj.contains_key(key),
                "German translations should contain key: {}",
                key
            );
        }

        // Test that translations are working through the Translation trait
        assert_eq!(
            config.t_with_lang("Language", &Language::from("de")),
            "Sprache"
        );
        assert_eq!(
            config.t_with_lang("Tasks", &Language::from("de")),
            "Aufgaben"
        );

        // Verify all supported languages have their translation files loaded
        for lang in Language::builtin() {
            assert!(
                config.translations.contains_key(lang.code()),
                "Translation file for {} should be loaded",
                lang.native_name()
            );
        }

        // Test the structure of loaded translations
        for (lang_code, translations) in &config.translations {
            assert!(
                translations.is_object(),
                "Translations for {} should be a JSON object",
                lang_code
            );
            assert!(
                !translations.as_object().unwrap().is_empty(),
                "Translations for {} should not be empty",
                lang_code
            );
        }
    }

    #[test]
    fn test_combined_platform_and_local_translations() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .is_test(true)
            .try_init();

        let config = create_i18n_config();

        // Test local translations (from your app)
        let local_keys = [
            "Please select a language from the dropdown.",
            "Please select a language:",
            "Select Language",
            "Tasks",
            "Unlock Points",
            "Rate this Challenge",
            "Submit",
            "Submitting...",
            "Leave a comment",
            "Thank you for your review!",
            "Name",
            "Performance",
            "Time",
            "Rank",
            "Show Tour Button",
            "Start Tour",
            "Feedback",
            "We'd love your feedback!",
        ];

        // Test platform translations (from konnektoren-platform)
        let platform_keys = ["Language"];

        // Test German translations
        let de_translations = config
            .translations
            .get("de")
            .expect("German translations should exist");

        let de_obj = de_translations.as_object().expect("Should be an object");

        // Verify local translations
        for &key in &local_keys {
            assert!(
                de_obj.contains_key(key),
                "Local translation key '{}' should exist in German translations",
                key
            );
        }

        // Verify platform translations
        for &key in &platform_keys {
            assert!(
                de_obj.contains_key(key),
                "Platform translation key '{}' should exist in German translations",
                key
            );
        }

        // Log all available keys for debugging
        log::info!("Available translation keys for German:");
        if let Some(obj) = de_translations.as_object() {
            for key in obj.keys() {
                log::info!("  {}", key);
            }
        }

        // Test that translations don't override each other
        let en_translations = config
            .translations
            .get("en")
            .expect("English translations should exist");

        let en_obj = en_translations.as_object().expect("Should be an object");

        // Verify translation count
        let total_keys = de_obj.len();
        log::info!("Total number of translations in German: {}", total_keys);
        assert!(
            total_keys >= local_keys.len() + platform_keys.len(),
            "Should have at least the sum of local and platform translations. Expected at least {}, got {}",
            local_keys.len() + platform_keys.len(),
            total_keys
        );

        // Test merged translations work through the Translation trait
        for lang in Language::builtin() {
            // Test some local keys
            for &key in &local_keys {
                let result = config.t_with_lang(key, &lang);
                assert!(
                    !result.is_empty(),
                    "Translation for '{}' should exist in {}",
                    key,
                    lang.native_name()
                );
            }

            // Test platform keys
            for &key in &platform_keys {
                let result = config.t_with_lang(key, &lang);
                log::info!(
                    "Platform translation for '{}' in {}: '{}'",
                    key,
                    lang.native_name(),
                    result
                );
            }
        }

        // Log final statistics
        log::info!("Translation test summary:");
        log::info!("  Local keys: {}", local_keys.len());
        log::info!("  Platform keys: {}", platform_keys.len());
        log::info!("  Total keys in German: {}", total_keys);
    }
}
