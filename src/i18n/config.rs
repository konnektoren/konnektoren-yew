use super::I18n;
use crate::i18n::LANGUAGES;
pub use konnektoren_platform::i18n::I18nConfig;
use konnektoren_platform::i18n::{CombinedTranslationAsset, Language, TranslationAsset};
use rust_embed::RustEmbed;
use serde_json::Value;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "src/assets/i18n/"]
pub struct LocalI18nAssets;

pub fn create_i18n_config() -> I18nConfig {
    I18nConfig::with_assets(CombinedTranslationAsset::<LocalI18nAssets>::new("i18n.yml"))
}

pub fn create_i18n() -> I18n {
    I18n::from(I18nConfig::with_assets(CombinedTranslationAsset::<
        LocalI18nAssets,
    >::new("i18n.yml")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use konnektoren_platform::prelude::Language;
    use serde_json::json;

    #[test]
    fn test_create_i18n() {
        let i18n = create_i18n();
        assert!(i18n.translations.contains_key("en"));
        assert!(i18n.translations.contains_key("de"));
        assert_eq!(i18n.default_language, Language::default());
    }

    #[test]
    fn test_translations() {
        let i18n = create_i18n();

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
        let i18n = create_i18n();
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
        let i18n = create_i18n();
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
}
