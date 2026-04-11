#[cfg(feature = "csr")]
use log::{debug, warn};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ReadTextProps {
    pub text: String,
    #[prop_or("en-US".to_string())]
    pub lang: String,
}

#[function_component(ReadText)]
pub fn read_text(props: &ReadTextProps) -> Html {
    #[cfg(feature = "csr")]
    {
        use crate::providers::use_settings;
        use gloo::timers::callback::Timeout;
        use gloo::utils::window;
        use web_sys::SpeechSynthesisUtterance;

        let settings = use_settings();

        let text_clone = props.text.clone();
        let lang_clone = props.lang.clone();
        use_effect_with(props.text.clone(), move |_| {
            let settings = settings.clone();

            // Resolve speech synthesis once so both the speak and cleanup share it
            let speech_synthesis = window().speech_synthesis().ok();
            if speech_synthesis.is_none() {
                warn!("Speech synthesis not available in this browser");
            }

            if let Some(ss) = speech_synthesis.clone() {
                // Cancel any queued or ongoing speech before starting the new one
                ss.cancel();

                Timeout::new(0, move || {
                    match SpeechSynthesisUtterance::new() {
                        Ok(utterance) => {
                            utterance.set_text(&text_clone);
                            utterance.set_lang(&lang_clone);
                            utterance.set_volume(settings.sound_volume);
                            ss.speak(&utterance);
                            debug!("Requested text-to-speech for: {}", text_clone);
                        }
                        Err(err) => {
                            debug!("Failed to create speech utterance: {:?}", err);
                        }
                    }
                })
                .forget();
            }

            // Cancel speech when text changes or component unmounts
            move || {
                if let Some(ss) = speech_synthesis {
                    ss.cancel();
                }
            }
        });
    }

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
