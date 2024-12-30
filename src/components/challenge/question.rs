use crate::components::TranslateComponent;
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
    let image = if let Some(image) = &props.question.image {
        if image.starts_with("fa-") {
            html! {
                <i class={classes!("question__icon", format!("fas {}", image))}></i>
            }
        } else {
            html! {
                <img src={image.to_string()} class="question__image" alt="" />
            }
        }
    } else {
        html! {}
    };

    html! {
        <div class="question">
            <h2 class="question__title">{"Question"}</h2>
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
