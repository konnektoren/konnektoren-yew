use konnektoren_platform::i18n::{I18nConfig, Language};
use std::ops::{Deref, DerefMut};

#[derive(Clone, PartialEq)]
pub struct I18n(I18nConfig);

impl I18n {
    pub fn t(&self, key: &str) -> String {
        self.0.get_translation(key, None)
    }

    pub fn t_with_lang(&self, key: &str, lang: &Language) -> String {
        self.0.get_translation(key, Some(lang))
    }
}

impl Deref for I18n {
    type Target = I18nConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for I18n {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<I18nConfig> for I18n {
    fn from(config: I18nConfig) -> Self {
        I18n(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_test_config() -> I18nConfig {
        let mut config = I18nConfig::default();
        let en_trans = json!({
            "Test": "Test",
            "Hello": "Hello"
        });
        let de_trans = json!({
            "Test": "Test",
            "Hello": "Hallo"
        });
        config.merge_translation(&Language::from("en"), en_trans);
        config.merge_translation(&Language::from("de"), de_trans);
        config
    }

    #[test]
    fn test_t_method() {
        let i18n = I18n::from(create_test_config());
        assert_eq!(i18n.t("Test"), "Test");
        assert_eq!(i18n.t("Hello"), "Hello");
        assert_eq!(i18n.t("NonExistent"), "NonExistent");
    }

    #[test]
    fn test_t_with_lang() {
        let i18n = I18n::from(create_test_config());
        assert_eq!(i18n.t_with_lang("Hello", &Language::from("de")), "Hallo");
        assert_eq!(i18n.t_with_lang("Hello", &Language::from("en")), "Hello");
    }

    #[test]
    fn test_deref() {
        let i18n = I18n::from(create_test_config());
        assert_eq!(
            i18n.get_translation("Hello", Some(&Language::from("de"))),
            "Hallo"
        );
    }

    #[test]
    fn test_deref_mut() {
        let mut i18n = I18n::from(create_test_config());
        let new_trans = json!({
            "NewKey": "New Value"
        });

        i18n.merge_translation(&Language::from("en"), new_trans);
        assert_eq!(i18n.t("NewKey"), "New Value");
    }

    #[test]
    fn test_from_config() {
        let config = create_test_config();
        let i18n = I18n::from(config.clone());

        assert_eq!(i18n.default_language, config.default_language);
        assert_eq!(i18n.translations, config.translations);
    }

    #[test]
    fn test_merge_translations() {
        let mut i18n = I18n::from(create_test_config());

        // Add new translations
        let new_trans = json!({
            "New": "New",
            "Test": "Override"
        });
        i18n.merge_translation(&Language::from("en"), new_trans);

        // Test new translation
        assert_eq!(i18n.t("New"), "New");
        // Test overridden translation
        assert_eq!(i18n.t("Test"), "Override");
        // Test existing translation
        assert_eq!(i18n.t("Hello"), "Hello");
    }

    #[test]
    fn test_multiple_languages() {
        let mut i18n = I18n::from(create_test_config());

        // Add translations for multiple languages
        let en_trans = json!({"New": "New"});
        let de_trans = json!({"New": "Neu"});

        i18n.merge_translation(&Language::from("en"), en_trans);
        i18n.merge_translation(&Language::from("de"), de_trans);

        assert_eq!(i18n.t_with_lang("New", &Language::from("en")), "New");
        assert_eq!(i18n.t_with_lang("New", &Language::from("de")), "Neu");
    }

    #[test]
    fn test_fallback_behavior() {
        let i18n = I18n::from(create_test_config());

        // Test fallback for unknown language
        assert_eq!(
            i18n.t_with_lang("Hello", &Language::from("fr")),
            "Hello" // Should fall back to default (en)
        );

        // Test fallback for unknown key
        assert_eq!(
            i18n.t_with_lang("Unknown", &Language::from("de")),
            "Unknown"
        );
    }
}
