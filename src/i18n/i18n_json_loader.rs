use crate::i18n::i18n_loader::I18nLoader;

pub struct I18nJsonLoader {
    pub data: serde_json::Value,
}

impl I18nJsonLoader {
    pub fn new(json_data: &str) -> Self {
        let data = serde_json::from_str::<serde_json::Value>(json_data).unwrap();
        I18nJsonLoader { data }
    }
}

impl I18nLoader for I18nJsonLoader {
    fn get(&self, _language: &str) -> Option<serde_json::Value> {
        self.get_all()
    }

    fn get_all(&self) -> Option<serde_json::Value> {
        Some(self.data.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all() {
        let json_data = include_str!("../assets/i18n/en.json");
        let loader = I18nJsonLoader::new(json_data);
        let data = loader.get_all();
        assert!(data.is_some());
    }
}
