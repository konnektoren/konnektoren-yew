use crate::i18n::LANGUAGE_KEY;
use gloo::storage::{LocalStorage, Storage};

#[derive(Clone, PartialEq)]
pub struct SelectedLanguage {
    pub language: String,
}

impl Default for SelectedLanguage {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
        }
    }
}

impl SelectedLanguage {
    pub fn new(language: &str) -> Self {
        Self {
            language: language.to_string(),
        }
    }

    pub fn set(&mut self, language: &str) {
        let _ = LocalStorage::set(LANGUAGE_KEY, &language);
    }

    pub fn get(&self) -> String {
        LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| self.language.clone())
    }
}
