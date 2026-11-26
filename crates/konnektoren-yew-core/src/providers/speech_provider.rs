use crate::model::SpeechConfig;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SpeechProviderProps {
    #[prop_or_default]
    pub config: SpeechConfig,
    pub children: Children,
}

#[function_component(SpeechProvider)]
pub fn speech_provider(props: &SpeechProviderProps) -> Html {
    html! {
        <ContextProvider<SpeechConfig> context={props.config.clone()}>
            {props.children.clone()}
        </ContextProvider<SpeechConfig>>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use crate::components::ReadText;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        SpeechProvider,
        SpeechProviderProps {
            config: SpeechConfig {
                volume: 0.5,
                voice: None
            },
            children: Children::new(vec![
                html! { <ReadText text="Hallo, Welt!" lang="de-DE" /> }
            ]),
        },
    );
}
