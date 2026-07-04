use super::FaqItem;
use konnektoren_rs::faq::FaqData;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FaqListProps {
    pub faq_data: FaqData,
    #[prop_or_else(|| "en".to_string())]
    pub language: String,
    #[prop_or_default]
    pub highlighted_id: Option<String>,
}

#[function_component(FaqList)]
pub fn faq_list(props: &FaqListProps) -> Html {
    html! {
        <section class="faq-list">
            {for props.faq_data.faqs.iter().cloned().map(|faq| {
                let highlighted = props.highlighted_id.as_deref() == Some(faq.id.as_str());
                html! {
                    <FaqItem
                        faq={faq}
                        language={props.language.clone()}
                        highlighted={highlighted}
                    />
                }
            })}
        </section>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    fn preview_faq_data() -> FaqData {
        serde_yaml::from_str(include_str!("../../../assets/faqs.yml")).unwrap_or_default()
    }

    yew_preview::create_preview!(
        FaqList,
        FaqListProps {
            faq_data: preview_faq_data(),
            language: "de".to_string(),
            highlighted_id: Some("general-what-is-konnektoren".to_string()),
        },
        (
            "highlighted theme",
            FaqListProps {
                faq_data: preview_faq_data(),
                language: "de".to_string(),
                highlighted_id: Some("general-theme-switch".to_string()),
            }
        ),
        (
            "not highlighted",
            FaqListProps {
                faq_data: preview_faq_data(),
                language: "de".to_string(),
                highlighted_id: None,
            }
        ),
        (
            "english highlighted",
            FaqListProps {
                faq_data: preview_faq_data(),
                language: "en".to_string(),
                highlighted_id: Some("general-theme-switch".to_string()),
            }
        ),
    );
}
