#[derive(Clone, PartialEq, Debug)]
pub enum Design {
    Desktop,
    Mobile,
    Other(String),
}

#[cfg(feature = "csr")]
const DESIGN_PROVIDER_ATTR: &str = "data-design-provider";
#[cfg(feature = "csr")]
const DESIGN_PROVIDER_MANAGED: &str = "managed";

/// How the active [`Design`] is chosen: follow the screen size, or a
/// user-configured fixed design.
#[derive(Clone, PartialEq, Debug, Default)]
pub enum DesignMode {
    #[default]
    Auto,
    Fixed(Design),
}

impl DesignMode {
    pub fn as_str(&self) -> String {
        match self {
            DesignMode::Auto => "auto".to_string(),
            DesignMode::Fixed(design) => design.as_class().to_string(),
        }
    }

    pub fn from_storage_value(value: &str) -> Self {
        match value {
            "" | "auto" => DesignMode::Auto,
            class => DesignMode::Fixed(Design::from_class(class)),
        }
    }
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

    /// Writes this design's class to `<body>`, replacing any other `design-*` class.
    #[cfg(feature = "csr")]
    pub fn apply_to_body(&self) {
        if let Some(body) = gloo::utils::document().body() {
            let mut classes: Vec<String> = body
                .class_name()
                .split_whitespace()
                .filter(|class| !class.starts_with("design-"))
                .map(String::from)
                .collect();
            classes.push(self.as_class().to_string());
            body.set_class_name(&classes.join(" "));
            if let Err(err) = body.set_attribute(DESIGN_PROVIDER_ATTR, DESIGN_PROVIDER_MANAGED) {
                tracing::warn!("Failed to mark design body class as provider-managed: {err:?}");
            }
        }
    }

    #[cfg(feature = "csr")]
    pub fn configured_from_body() -> Option<Self> {
        let body = gloo::utils::document().body()?;
        if body
            .get_attribute(DESIGN_PROVIDER_ATTR)
            .as_deref()
            .is_some_and(|value| value == DESIGN_PROVIDER_MANAGED)
        {
            return None;
        }

        let class_list = body.class_name();
        class_list
            .split_whitespace()
            .find(|class| class.starts_with("design-"))
            .map(Design::from_class)
    }

    #[cfg(not(feature = "csr"))]
    pub fn configured_from_body() -> Option<Self> {
        None
    }

    pub fn get_from_body() -> Self {
        #[cfg(feature = "csr")]
        {
            Design::configured_from_body().unwrap_or(Design::Desktop)
        }

        #[cfg(not(feature = "csr"))]
        {
            Design::Desktop
        }
    }
}
