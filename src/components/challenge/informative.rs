use konnektoren_core::challenges::{ChallengeResult, Informative};
use konnektoren_core::commands::{ChallengeCommand, Command};
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct InformativeComponentProps {
    pub challenge: Informative,
    #[prop_or_default]
    pub on_command: Option<Callback<Command>>,
    #[prop_or_default]
    pub language: Option<String>,
}

#[function_component(InformativeComponent)]
pub fn informative_component(props: &InformativeComponentProps) -> Html {
    let language = props.language.as_deref().unwrap_or("en");

    let on_command = props.on_command.clone();
    let on_finish = Callback::from(move |_| {
        if let Some(on_command) = on_command.as_ref() {
            let command =
                Command::Challenge(ChallengeCommand::Finish(Some(ChallengeResult::Informative)));
            on_command.emit(command);
        }
    });

    let informative_text = props.challenge.text.iter().find(|t| t.language == language);

    let text = match informative_text {
        Some(text) => &text.text,
        None => "No text found",
    };

    html! {
        <div class="informative">
            <h1>{&props.challenge.description}</h1>
            <p>{text}</p>
            <button onclick={on_finish}>{"Finish"}</button>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::InformativeText;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        InformativeComponent,
        InformativeComponentProps {
            challenge: Informative {
                id: "".to_string(),
                name: "".to_string(),
                description: "Informative Challenge".to_string(),
                text: vec![InformativeText {
                    language: "en".to_string(),
                    text: "This is an informative challenge".to_string(),
                }],
            },
            on_command: None,
            language: None,
        },
        (
            "unknown language",
            InformativeComponentProps {
                challenge: Informative {
                    id: "".to_string(),
                    name: "".to_string(),
                    description: "Informative Challenge".to_string(),
                    text: vec![InformativeText {
                        language: "en".to_string(),
                        text: "This is an informative challenge".to_string(),
                    }],
                },
                on_command: None,
                language: Some("de".to_string()),
            }
        )
    );
}
