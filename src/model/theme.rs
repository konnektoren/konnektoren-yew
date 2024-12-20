use gloo::utils::document;

#[derive(Clone, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
    Star,
    Other(String),
}

impl Theme {
    pub fn as_class(&self) -> &str {
        match self {
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
            Theme::Star => "theme-star",
            Theme::Other(class) => class,
        }
    }

    pub fn from_class(class: &str) -> Self {
        match class {
            "theme-light" => Theme::Light,
            "theme-dark" => Theme::Dark,
            "theme-star" => Theme::Star,
            _ => Theme::Other(class.to_string()),
        }
    }

    pub fn get_from_body() -> Self {
        let class_list = document().body().unwrap().class_name();
        let theme_class = class_list
            .split_whitespace()
            .find(|class| class.starts_with("theme-"))
            .unwrap_or("theme-light");

        Theme::from_class(theme_class)
    }
}
