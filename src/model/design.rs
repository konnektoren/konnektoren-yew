use gloo::utils::document;

#[derive(Clone, PartialEq, Debug)]
pub enum Design {
    Desktop,
    Mobile,
    Other(String),
}

impl Design {
    pub fn as_class(&self) -> &str {
        match self {
            Design::Desktop => "desktop-design",
            Design::Mobile => "mobile-design",
            Design::Other(class) => class,
        }
    }

    pub fn from_class(class: &str) -> Self {
        match class {
            "desktop-design" => Design::Desktop,
            "mobile-design" => Design::Mobile,
            _ => Design::Other(class.to_string()),
        }
    }

    pub fn get_from_body() -> Self {
        let class_list = document().body().unwrap().class_name();
        let design_class = class_list
            .split_whitespace()
            .find(|class| class.ends_with("-design"))
            .unwrap_or("desktop-design");

        Design::from_class(design_class)
    }
}
