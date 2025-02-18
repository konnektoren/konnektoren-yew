use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub id: String,
    pub language: String,
    pub music_volume: f32,
    pub sound_volume: f32,
    pub theme: String,
    pub show_helpers: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            language: "en".to_string(),
            music_volume: 0.0,
            sound_volume: 0.8,
            theme: "light".to_string(),
            show_helpers: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.language, "en");
        assert_eq!(settings.music_volume, 0.0);
        assert_eq!(settings.sound_volume, 0.8);
        assert_eq!(settings.theme, "light");
        assert_eq!(settings.show_helpers, true);
    }

    #[test]
    fn ser_de_settings() {
        let settings = Settings::default();
        let json = serde_json::to_string(&settings).unwrap();
        let settings2: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, settings2);
    }
}
