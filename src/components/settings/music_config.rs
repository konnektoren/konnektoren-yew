use crate::model::Settings;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MusicConfigProps {
    pub settings: Settings,
    pub on_change: Callback<Settings>,
}

#[function_component(MusicConfig)]
pub fn music_config(props: &MusicConfigProps) -> Html {
    let on_change = props.on_change.clone();
    let music_volume = props.settings.music_volume;
    let on_change_music_volume = {
        let settings = props.settings.clone();
        Callback::from(move |e: InputEvent| {
            let music_volume = {
                let input: HtmlInputElement = e.target_unchecked_into();
                input.value().parse().unwrap_or(0.0)
            };
            on_change.emit(Settings {
                music_volume,
                ..settings.clone()
            });
        })
    };

    html! {
        <div class="music-config">
            <label for="music-volume">{"Music Volume"}</label>
            <input
                id="music-volume"
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={music_volume.to_string()}
                oninput={on_change_music_volume}
            />
            <output>{music_volume}</output>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        MusicConfig,
        MusicConfigProps {
            settings: Settings {
                music_volume: 0.5,
                ..Settings::default()
            },
            on_change: Callback::noop()
        },
    );
}
