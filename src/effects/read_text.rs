use crate::providers::use_settings;
use gloo::timers::callback::Timeout;
use gloo::utils::window;
use web_sys::SpeechSynthesisUtterance;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ReadTextProps {
    pub text: String,
    #[prop_or("en-US".to_string())]
    pub lang: String,
}

#[function_component(ReadText)]
pub fn read_text(props: &ReadTextProps) -> Html {
    let settings = use_settings();

    let text_clone = props.text.clone();
    let lang_clone = props.lang.clone();
    use_effect(move || {
        let settings = settings.clone();
        Timeout::new(0, move || {
            if let Ok(speech_synthesis) = window().speech_synthesis() {
                let utterance = SpeechSynthesisUtterance::new().unwrap();
                utterance.set_text(&text_clone);
                utterance.set_lang(&lang_clone);
                utterance.set_volume(settings.sound_volume);

                speech_synthesis.speak(&utterance);
            }
        })
        .forget();
        || ()
    });

    html! {
        <>
        </>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ReadText,
        ReadTextProps {
            text: "Hello, World!".to_string(),
            lang: "en-US".to_string()
        },
        (
            "german",
            ReadTextProps {
                text: "Hallo, Welt!".to_string(),
                lang: "de-DE".to_string()
            }
        ),
        (
            "french",
            ReadTextProps {
                text: "Bonjour le monde!".to_string(),
                lang: "fr-FR".to_string()
            }
        )
    );
}
