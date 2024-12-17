use crate::i18n::i18n_loader::I18nLoader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct YmlFile {
    i18n: HashMap<String, HashMap<String, String>>,
}

pub struct I18nYmlLoader {
    pub i18n: HashMap<String, HashMap<String, String>>,
}

impl I18nYmlLoader {
    pub fn new(yml_data: &str) -> Self {
        let i18n = serde_yaml::from_str::<YmlFile>(yml_data).unwrap().i18n;
        I18nYmlLoader { i18n }
    }
}

impl I18nLoader for I18nYmlLoader {
    fn get(&self, language: &str) -> Option<Value> {
        let mut map = HashMap::new();
        for (key, value) in &self.i18n {
            map.insert(key.clone(), value.get(language).unwrap_or(key).clone());
        }
        Some(serde_json::to_value(map).unwrap())
    }

    fn get_all(&self) -> Option<Value> {
        self.get("en")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all() {
        let loader = I18nYmlLoader::new("i18n:\n  hello:\n    en: Hello\n    de: Hallo");
        let data = loader.get_all();
        assert!(data.is_some());
    }
}
