use crate::components::{Icon, TranslateComponent};
use crate::i18n::use_i18n;
use konnektoren_core::challenges::Question;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct QuestionComponentProps {
    pub question: Question,
    #[prop_or_default]
    pub help: bool,
}

#[function_component(QuestionComponent)]
pub fn question_component(props: &QuestionComponentProps) -> Html {
    let i18n = use_i18n();

    let image = if let Some(image) = &props.question.image {
        html! { <Icon icon={image.clone()} class="question__icon" image_class={Some("question__image".to_owned())} alt="" fallback_as_image={true} /> }
    } else {
        html! {}
    };

    html! {
        <div class="question">
            <h2 class="question__title">{ i18n.t("Question") }</h2>
            {image}
            <p class="question__text">{&props.question.question}</p>
            if props.help {
                <div class="question__help">
                    <p class="question__help-text">{&props.question.help}</p>
                    <div class="question__help-translation">
                        <TranslateComponent text={props.question.help.to_string()} />
                    </div>
                </div>
            }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        QuestionComponent,
        QuestionComponentProps {
            question: Question {
                question: "What is the capital of Germany?".to_string(),
                help: "Berlin".to_string(),
                image: None,
                option: 0,
            },
            help: false,
        },
        (
            "With Image",
            QuestionComponentProps {
                question: Question {
                    question: "What is the capital of Germany?".to_string(),
                    help: "Berlin".to_string(),
                    image: Some(
                        "https://www.rust-lang.org/logos/rust-logo-512x512.png".to_string()
                    ),
                    option: 0,
                },
                help: false,
            }
        ),
        (
            "With Help",
            QuestionComponentProps {
                question: Question {
                    question: "What is the capital of Germany?".to_string(),
                    help: "Berlin".to_string(),
                    image: None,
                    option: 0,
                },
                help: true,
            }
        )
    );
}
