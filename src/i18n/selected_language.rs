use crate::i18n::LANGUAGE_KEY;
use konnektoren_platform::i18n::Language;

#[derive(Clone, Default, PartialEq)]
pub struct SelectedLanguage {
    pub language: Language,
}

impl SelectedLanguage {
    pub fn new(language: &str) -> Self {
        Self {
            language: language.into(),
        }
    }

    pub fn set(&mut self, code: &str) {
        #[cfg(not(feature = "ssr"))]
        {
            use gloo::storage::{LocalStorage, Storage};
            let _ = LocalStorage::set(LANGUAGE_KEY, code);
        }
        self.language = Language::from_code(code);
    }

    pub fn get(&self) -> Language {
        #[cfg(not(feature = "ssr"))]
        {
            use gloo::storage::{LocalStorage, Storage};
            let code: Result<String, _> = LocalStorage::get(LANGUAGE_KEY);
            if let Ok(code) = code {
                return Language::from_code(&code);
            }
        }

        // Default behavior for SSR or when storage fails
        // Try to get from LANGUAGE environment variable if in SSR mode
        #[cfg(feature = "ssr")]
        {
            if let Ok(lang) = std::env::var("LANGUAGE") {
                return Language::from_code(&lang);
            }
        }

        // Fallback to current language
        self.language.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(feature = "ssr"))]
    use gloo::storage::{LocalStorage, Storage};
    #[cfg(not(feature = "ssr"))]
    use wasm_bindgen_test::*;

    #[cfg(not(feature = "ssr"))]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_default_language() {
        let selected = SelectedLanguage::default();
        assert_eq!(selected.language.code(), "en");
    }

    #[test]
    fn test_new_language() {
        let selected = SelectedLanguage::new("de");
        assert_eq!(selected.language.code(), "de");
    }

    #[test]
    fn test_invalid_language_code() {
        let selected = SelectedLanguage::new("invalid");
        assert_eq!(selected.language.code(), "en");
    }

    #[cfg(not(feature = "ssr"))]
    #[wasm_bindgen_test]
    async fn test_set_and_get_language() {
        LocalStorage::clear();

        let mut selected = SelectedLanguage::default();
        selected.set("de");
        assert_eq!(selected.get().code(), "de");

        LocalStorage::clear();
    }
}
