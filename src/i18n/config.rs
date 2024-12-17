use crate::i18n::i18n_json_loader::I18nJsonLoader;
use crate::i18n::i18n_loader::I18nLoader;
use crate::i18n::LANGUAGES;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct I18nConfig {
    pub supported_languages: Vec<&'static str>,

    pub translations: HashMap<String, serde_json::Value>,

    pub default_language: String,
}

impl Default for I18nConfig {
    fn default() -> Self {
        let supported_languages = LANGUAGES.to_vec();
        let mut translations = HashMap::new();

        let i18n_data = [
            ("ar", include_str!("../assets/i18n/ar.json")),
            ("cn", include_str!("../assets/i18n/cn.json")),
            ("en", include_str!("../assets/i18n/en.json")),
            ("de", include_str!("../assets/i18n/de.json")),
            ("es", include_str!("../assets/i18n/es.json")),
            ("pl", include_str!("../assets/i18n/pl.json")),
            ("tr", include_str!("../assets/i18n/tr.json")),
            ("ua", include_str!("../assets/i18n/ua.json")),
            ("vi", include_str!("../assets/i18n/vi.json")),
        ];
        for (lang, data) in i18n_data.iter() {
            let loader = I18nJsonLoader::new(data);
            let json_data = loader.get_all().unwrap();
            translations.insert(lang.to_string(), json_data);
        }

        Self {
            supported_languages,
            translations,
            default_language: "en".to_string(),
        }
    }
}

impl I18nConfig {
    pub fn new(
        supported_languages: Vec<&'static str>,
        translations: HashMap<String, serde_json::Value>,
        default_language: String,
    ) -> Self {
        Self {
            supported_languages,
            translations,
            default_language,
        }
    }

    pub fn get_translation(&self, text: &str, lang: Option<&str>) -> String {
        let language = lang
            .map(|l| l.to_string())
            .unwrap_or_else(|| self.default_language.clone());

        let translation = self
            .translations
            .get(&language)
            .unwrap_or(&serde_json::Value::Null);

        translation[text].as_str().unwrap_or(text).to_string()
    }

    pub fn merge_translation(&mut self, lang: &str, translation: serde_json::Value) {
        match self.translations.get(lang) {
            Some(existing) => {
                let mut merged = existing.clone();
                merged
                    .as_object_mut()
                    .unwrap()
                    .extend(translation.as_object().unwrap().clone());
                self.translations.insert(lang.to_string(), merged);
            }
            None => {
                self.translations.insert(lang.to_string(), translation);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_default() {
        let config = I18nConfig::default();

        assert_eq!(config.supported_languages.len(), 9);
        assert_eq!(config.translations.len(), 9);
        assert_eq!(config.default_language, "en");
    }

    #[test]
    fn test_get_translation() {
        let mut translations = HashMap::new();
        translations.insert(
            "en".to_string(),
            json!({ "Hello": "Hello", "World": "World" }),
        );

        let config = I18nConfig::new(vec!["en"], translations, "en".to_string());

        assert_eq!(config.get_translation("Hello", None), "Hello");
        assert_eq!(config.get_translation("World", None), "World");
        assert_eq!(config.get_translation("Hello", Some("en")), "Hello");
        assert_eq!(config.get_translation("World", Some("en")), "World");
        assert_eq!(config.get_translation("Hello", Some("de")), "Hello");
        assert_eq!(config.get_translation("World", Some("de")), "World");
    }

    #[test]
    fn test_get_translation_default() {
        let mut translations = HashMap::new();
        translations.insert(
            "en".to_string(),
            json!({ "Hello": "Hello", "World": "World" }),
        );

        let config = I18nConfig::new(vec!["en"], translations, "en".to_string());

        assert_eq!(config.get_translation("Hello", None), "Hello");
        assert_eq!(config.get_translation("World", None), "World");
        assert_eq!(config.get_translation("Hello", Some("en")), "Hello");
        assert_eq!(config.get_translation("World", Some("en")), "World");
        assert_eq!(config.get_translation("Hello", Some("de")), "Hello");
        assert_eq!(config.get_translation("World", Some("de")), "World");
    }

    #[test]
    fn test_get_translation_missing() {
        let mut translations = HashMap::new();
        translations.insert("en".to_string(), json!({ "Hello": "Hello"}));

        let config = I18nConfig::new(vec!["en"], translations, "en".to_string());

        assert_eq!(config.get_translation("Hello", None), "Hello");
        assert_eq!(config.get_translation("World", None), "World");
        assert_eq!(config.get_translation("Hello", Some("en")), "Hello");
        assert_eq!(config.get_translation("World", Some("en")), "World");
        assert_eq!(config.get_translation("Hello", Some("de")), "Hello");
        assert_eq!(config.get_translation("World", Some("de")), "World");
    }

    #[test]
    fn test_get_translation_empty() {
        let translations = HashMap::new();

        let config = I18nConfig::new(vec!["en"], translations, "en".to_string());

        assert_eq!(config.get_translation("Hello", None), "Hello");
        assert_eq!(config.get_translation("World", None), "World");
        assert_eq!(config.get_translation("Hello", Some("en")), "Hello");
        assert_eq!(config.get_translation("World", Some("en")), "World");
        assert_eq!(config.get_translation("Hello", Some("de")), "Hello");
        assert_eq!(config.get_translation("World", Some("de")), "World");
    }

    #[test]
    fn test_get_translation_lang() {
        let mut translations = HashMap::new();
        translations.insert(
            "en".to_string(),
            json!({ "Hello": "Hello", "World": "World" }),
        );
        translations.insert(
            "de".to_string(),
            json!({ "Hello": "Hallo", "World": "Welt" }),
        );

        let config = I18nConfig::new(vec!["en", "de"], translations, "en".to_string());

        assert_eq!(config.get_translation("Hello", None), "Hello");
        assert_eq!(config.get_translation("Hello", Some("en")), "Hello");
        assert_eq!(config.get_translation("Hello", Some("de")), "Hallo");
        assert_eq!(config.get_translation("Hello", Some("es")), "Hello");
    }

    #[test]
    fn test_merge_translation() {
        let mut translations = HashMap::new();
        translations.insert("de".to_string(), json!({ "Hello": "Hello" }));

        let mut config = I18nConfig::new(vec!["en", "de"], translations, "en".to_string());

        let translation = json!({ "World": "Welt" });
        config.merge_translation("de", translation.clone());

        assert_eq!(config.translations.get("de").unwrap()["World"], "Welt");
    }
}
