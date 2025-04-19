use konnektoren_core::challenges::Placeholder;
use konnektoren_core::challenges::placeholder::PlaceholderType;
use konnektoren_core::commands::{ChallengeCommand, Command};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlaceholderComponentProps {
    pub challenge: Placeholder,
    #[prop_or_default]
    pub on_command: Option<Callback<Command>>,
    #[prop_or_default]
    pub language: Option<String>,
}

#[function_component(PlaceholderComponent)]
pub fn placeholder_component(props: &PlaceholderComponentProps) -> Html {
    let language = props.language.as_deref().unwrap_or("en");
    let challenge: Placeholder = props.challenge.clone();

    let on_command = props.on_command.clone();
    let on_finish = Callback::from(move |_| {
        if let Some(on_command) = on_command.as_ref() {
            let command = Command::Challenge(ChallengeCommand::Finish(None));
            on_command.emit(command);
        }
    });

    let placeholder_text = challenge
        .text
        .iter()
        .find(|t| t.language == language)
        .map(|t| t.text.clone())
        .unwrap_or_default();

    let (type_class, type_icon) = match challenge.type_ {
        PlaceholderType::ComingSoon => ("placeholder--coming-soon", "fa-solid fa-clock"),
        PlaceholderType::Planned => ("placeholder--planned", "fa-solid fa-calendar"),
        PlaceholderType::UnderDevelopment => ("placeholder--under-development", "fa-solid fa-code"),
    };

    let estimated_time = challenge.estimated_time.as_deref().unwrap_or("TBA");

    html! {
        <div class={classes!("placeholder", type_class)}>
            <div class="placeholder__header">
                <h1 class="placeholder__title">{&challenge.name}</h1>
                <div class="placeholder__type">
                    <i class={type_icon}></i>
                    {format!("{:?}", challenge.type_)}
                </div>
            </div>

            <div class="placeholder__content">
                <div class="placeholder__media">
                    {
                        if let Some(image) = challenge.image {
                            if image.starts_with("fa-") {
                                html! {
                                    <i class={classes!("fa-solid", image, "placeholder__icon-image")}></i>
                                }
                            } else {
                                html! {
                                    <img src={image} class="placeholder__image" alt="" />
                                }
                            }
                        } else {
                            html! {
                                <i class="fa-solid fa-question-circle placeholder__default-icon"></i>
                            }
                        }
                    }
                </div>

                <div class="placeholder__info">
                    <p class="placeholder__description">{&challenge.description}</p>
                    <div class="placeholder__meta">
                        <span class="placeholder__time">
                            <i class="fa-solid fa-hourglass-half"></i>
                            {estimated_time}
                        </span>
                    </div>
                </div>

                <div class="placeholder__text markdown-body">
                    {Html::from_html_unchecked(AttrValue::from(markdown::to_html(&placeholder_text)))}
                </div>
            </div>

            <div class="placeholder__actions">
                <button class="placeholder__button" onclick={on_finish}>
                    {"Continue"}
                </button>
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        PlaceholderComponent,
        PlaceholderComponentProps {
            challenge: Placeholder::default(),
            on_command: None,
            language: None,
        },
        (
            "German",
            PlaceholderComponentProps {
                challenge: Placeholder::default(),
                on_command: None,
                language: Some("de".to_string()),
            }
        ),
        (
            "Planned",
            PlaceholderComponentProps {
                challenge: Placeholder {
                    id: "placeholder-planned".to_string(),
                    name: "Planned Placeholder".to_string(),
                    description: "This is a planned placeholder.".to_string(),
                    type_: PlaceholderType::Planned,
                    image: None,
                    estimated_time: Some("Q3 2022".to_string()),
                    text: vec![],
                },
                on_command: None,
                language: None,
            }
        ),
        (
            "Under Development",
            PlaceholderComponentProps {
                challenge: Placeholder {
                    id: "placeholder-dev".to_string(),
                    name: "Under Development: New Feature".to_string(),
                    description: "This feature is currently being developed.".to_string(),
                    type_: PlaceholderType::UnderDevelopment,
                    image: Some("fa-solid fa-code".to_string()),
                    estimated_time: Some("Q4 2023".to_string()),
                    text: vec![],
                },
                on_command: None,
                language: None,
            }
        ),
        (
            "Image",
            PlaceholderComponentProps {
                challenge: Placeholder {
                    id: "placeholder-image".to_string(),
                    name: "Image Placeholder".to_string(),
                    description: "This is a placeholder with an image.".to_string(),
                    type_: PlaceholderType::ComingSoon,
                    image: Some("https://via.placeholder.com/300".to_string()),
                    estimated_time: Some("TBA".to_string()),
                    text: vec![],
                },
                on_command: None,
                language: None,
            }
        )
    );
}
