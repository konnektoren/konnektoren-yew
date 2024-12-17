pub trait I18nLoader {
    fn get(&self, language: &str) -> Option<serde_json::Value>;
    fn get_all(&self) -> Option<serde_json::Value>;
}
