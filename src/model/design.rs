#[derive(Clone, PartialEq, Debug)]
pub enum Design {
    Desktop,
    Mobile,
    Other(String),
}

impl Design {
    pub fn as_class(&self) -> &str {
        match self {
            Design::Desktop => "design-desktop",
            Design::Mobile => "design-mobile",
            Design::Other(class) => class,
        }
    }

    pub fn from_class(class: &str) -> Self {
        match class {
            "design-desktop" => Design::Desktop,
            "design-mobile" => Design::Mobile,
            _ => Design::Other(class.to_string()),
        }
    }

    pub fn get_from_body() -> Self {
        #[cfg(feature = "csr")]
        {
            let class_list = gloo::utils::document().body().unwrap().class_name();
            let design_class = class_list
                .split_whitespace()
                .find(|class| class.starts_with("design-"))
                .unwrap_or("design-desktop");

            Design::from_class(design_class)
        }

        #[cfg(not(feature = "csr"))]
        {
            Design::Desktop
        }
    }
}
