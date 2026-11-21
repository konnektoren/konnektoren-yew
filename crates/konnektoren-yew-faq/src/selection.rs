//! Selecting FAQs by id or tag, e.g. from the URL hash `#general-theme-switch`
//! or `#settings`.

use konnektoren_rs::faq::{Faq, FaqData};

/// What a URL hash selects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FaqSelection {
    /// A single FAQ by its id.
    Id(String),
    /// All FAQs with this tag.
    Tag(String),
}

impl FaqSelection {
    /// An existing FAQ id selects that FAQ; any other non-empty hash is a tag.
    pub fn from_hash(hash: &str, data: &FaqData) -> Option<Self> {
        let hash = normalized_hash(hash);
        if hash.is_empty() {
            None
        } else if data.faqs.iter().any(|faq| faq.id == hash) {
            Some(Self::Id(hash.to_string()))
        } else {
            Some(Self::Tag(hash.to_string()))
        }
    }

    /// The FAQs this selection matches, in YAML order.
    pub fn filter(&self, data: &FaqData) -> Vec<Faq> {
        data.faqs
            .iter()
            .filter(|faq| match self {
                Self::Id(id) => faq.id == *id,
                Self::Tag(tag) => faq.tags.iter().any(|faq_tag| faq_tag.as_str() == tag),
            })
            .cloned()
            .collect()
    }
}

/// The hash without its leading `#`.
pub fn normalized_hash(hash: &str) -> &str {
    hash.trim_start_matches('#')
}

/// The FAQ tag or id under a click inside a [`FaqList`](crate::FaqList), as hash
/// without `#`: a tag when a `.faq-tag` was clicked, otherwise the id of the
/// surrounding `.faq-item`.
#[cfg(feature = "csr")]
pub fn hash_from_click(event: &yew::MouseEvent) -> Option<String> {
    use wasm_bindgen::JsCast;

    let target = event.target()?.dyn_into::<web_sys::Element>().ok()?;

    if let Ok(Some(tag)) = target.closest(".faq-tag") {
        let tag = tag.text_content().unwrap_or_default();
        let tag = normalized_hash(tag.trim());
        return (!tag.is_empty()).then(|| tag.to_string());
    }

    if let Ok(Some(item)) = target.closest(".faq-item") {
        let id = item.id();
        return (!id.is_empty()).then_some(id);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const YAML: &str = r#"
faqs:
  - id: general-what-is-konnektoren
    timestamp: "2025-01-29T19:19:02+01:00"
    question: "What is Konnektoren?"
    answer: "A learning app."
    tags: ["general", "about"]
  - id: general-theme-switch
    timestamp: "2026-07-03T00:00:00+01:00"
    question: "Dark mode?"
    answer: "Yes."
    tags: ["general", "settings"]
"#;

    fn data() -> FaqData {
        serde_yaml::from_str(YAML).unwrap()
    }

    #[test]
    fn from_hash() {
        let data = data();
        assert_eq!(
            FaqSelection::from_hash("#general-theme-switch", &data),
            Some(FaqSelection::Id("general-theme-switch".into()))
        );
        assert_eq!(
            FaqSelection::from_hash("settings", &data),
            Some(FaqSelection::Tag("settings".into()))
        );
        assert_eq!(FaqSelection::from_hash("#", &data), None);
        assert_eq!(FaqSelection::from_hash("", &data), None);
    }

    #[test]
    fn filter() {
        let data = data();
        let ids = |selection: FaqSelection| -> Vec<String> {
            selection
                .filter(&data)
                .into_iter()
                .map(|faq| faq.id)
                .collect()
        };
        assert_eq!(
            ids(FaqSelection::Id("general-theme-switch".into())),
            ["general-theme-switch"]
        );
        assert_eq!(
            ids(FaqSelection::Tag("general".into())),
            ["general-what-is-konnektoren", "general-theme-switch"]
        );
        assert!(ids(FaqSelection::Tag("unknown".into())).is_empty());
    }
}
