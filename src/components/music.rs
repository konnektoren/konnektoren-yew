use crate::providers::use_settings;
use uuid::Uuid;
use yew::prelude::*;

const MUSIC_URL: &str = "https://konnektoren.help/assets/fanfare-3-rpg.ogg";

#[derive(Properties, Clone, PartialEq)]
pub struct MusicComponentProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub url: Option<String>,
    #[prop_or_default]
    pub repeat: Option<bool>,
}

impl Default for MusicComponentProps {
    fn default() -> Self {
        Self {
            id: None,
            url: Some(MUSIC_URL.to_string()),
            repeat: Some(true),
        }
    }
}

#[function_component(MusicComponent)]
pub fn music_component(props: &MusicComponentProps) -> Html {
    let settings = use_settings();

    let audio_ref = use_node_ref();

    #[cfg(feature = "csr")]
    {
        use web_sys::HtmlAudioElement;

        let audio_ref = audio_ref.clone();
        let music_url = props
            .url
            .clone()
            .unwrap_or(MusicComponentProps::default().url.unwrap());
        let repeat = props
            .repeat
            .unwrap_or(MusicComponentProps::default().repeat.unwrap());
        let settings = settings.clone();
        use_effect_with((audio_ref, settings), move |(audio_ref, settings)| {
            if let Some(audio_element) = audio_ref.cast::<HtmlAudioElement>() {
                audio_element.set_src(&music_url);
                audio_element.set_loop(repeat);
                audio_element.set_autoplay(true);
                audio_element.set_volume(settings.music_volume as f64);

                let _ = audio_element.play();
            }
            || ()
        });
    }

    let id = props.id.clone().unwrap_or(Uuid::new_v4().to_string());

    html! {
        <div {id} class="music-component">
            <audio ref={audio_ref} />
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        MusicComponent,
        MusicComponentProps {
            id: Some("music-component".to_string()),
            url: Some(MUSIC_URL.to_string()),
            repeat: Some(false),
        },
        (
            "repeat",
            MusicComponentProps {
                id: Some("music-component".to_string()),
                url: Some(MUSIC_URL.to_string()),
                repeat: Some(true),
            }
        )
    );
}
