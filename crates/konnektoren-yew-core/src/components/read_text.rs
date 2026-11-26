#[cfg(feature = "csr")]
use crate::model::SpeechConfig;
use yew::prelude::*;

#[cfg(feature = "csr")]
mod browser;

#[derive(Properties, PartialEq)]
pub struct ReadTextProps {
    pub text: String,
    #[prop_or_else(|| "en-US".to_owned())]
    pub lang: String,
}

/// Speaks the text while mounted, using the nearest provider or default settings.
#[function_component(ReadText)]
pub fn read_text(props: &ReadTextProps) -> Html {
    #[cfg(feature = "csr")]
    {
        let config = use_context::<SpeechConfig>().unwrap_or_default();
        use_effect_with(
            (props.text.clone(), props.lang.clone(), config),
            |(text, lang, config)| browser::speak(text, lang, config),
        );
    }

    #[cfg(not(feature = "csr"))]
    let _ = props;

    Html::default()
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ReadText,
        ReadTextProps {
            text: "Hello, World!".into(),
            lang: "en-US".into(),
        },
        (
            "German",
            ReadTextProps {
                text: "Hallo, Welt!".into(),
                lang: "de-DE".into(),
            }
        ),
    );
}
