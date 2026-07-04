use super::FaqTag;
use konnektoren_rs::faq::Faq;
use konnektoren_rs::i18n::Language;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FaqItemProps {
    pub faq: Faq,
    #[prop_or_else(|| "en".to_string())]
    pub language: String,
    #[prop_or_default]
    pub highlighted: bool,
}

#[function_component(FaqItem)]
pub fn faq_item(props: &FaqItemProps) -> Html {
    let language = Language::from_code(&props.language);
    let question = props.faq.get_localized_question(&language);
    let answer = props.faq.get_localized_answer(&language);

    html! {
        <article
            id={props.faq.id.clone()}
            class={classes!(
                "faq-item",
                props.highlighted.then_some("faq-item--highlighted"),
            )}
        >
            <h3 class="faq-item__question">{question}</h3>
            <p class="faq-item__answer">{answer}</p>
            if !props.faq.tags.is_empty() {
                <div class="faq-item__tags" aria-label="FAQ tags">
                    {for props.faq.tags.iter().map(|tag| html! {
                        <FaqTag tag={tag.as_str().to_string()} />
                    })}
                </div>
            }
        </article>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_rs::faq::FaqData;
    use yew_preview::prelude::*;

    fn fallback_faq() -> Faq {
        Faq {
            id: "faq-preview".to_string(),
            timestamp: chrono::Utc::now(),
            question: "What is Konnektoren?".to_string(),
            answer: "Konnektoren helps learners practice German connectors.".to_string(),
            tags: vec!["general".into()],
            i18n: Default::default(),
        }
    }

    fn preview_faq() -> Faq {
        serde_yaml::from_str::<FaqData>(include_str!("../../../assets/faqs.yml"))
            .ok()
            .and_then(|faq_data| faq_data.faqs.into_iter().next())
            .unwrap_or_else(fallback_faq)
    }

    yew_preview::create_preview!(
        FaqItem,
        FaqItemProps {
            faq: preview_faq(),
            language: "de".to_string(),
            highlighted: false,
        },
        (
            "highlighted",
            FaqItemProps {
                faq: preview_faq(),
                language: "de".to_string(),
                highlighted: true,
            }
        ),
        (
            "english",
            FaqItemProps {
                faq: preview_faq(),
                language: "en".to_string(),
                highlighted: false,
            }
        ),
        (
            "english highlighted",
            FaqItemProps {
                faq: preview_faq(),
                language: "en".to_string(),
                highlighted: true,
            }
        ),
    );
}
