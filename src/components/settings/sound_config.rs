use crate::i18n::use_i18n;
use crate::model::Settings;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SoundConfigProps {
    pub settings: Settings,
    pub on_change: Callback<Settings>,
}

#[function_component(SoundConfig)]
pub fn sound_config(props: &SoundConfigProps) -> Html {
    let i18n = use_i18n();
    let on_change = props.on_change.clone();
    let sound_volume = props.settings.sound_volume;
    let on_change_sound_volume = {
        let settings = props.settings.clone();
        Callback::from(move |e: InputEvent| {
            #[cfg(feature = "csr")]
            {
                use web_sys::HtmlInputElement;
                let input: HtmlInputElement = e.target_unchecked_into();
                let sound_volume = input.value().parse().unwrap_or(0.0);
                on_change.emit(Settings {
                    sound_volume,
                    ..settings.clone()
                });
            }
        })
    };

    html! {
        <div class="sound-config">
            <label for="sound-volume">{ i18n.t("Sound Volume") }</label>
            <input
                id="sound-volume"
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={sound_volume.to_string()}
                oninput={on_change_sound_volume}
            />
            <output>{sound_volume}</output>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        SoundConfig,
        SoundConfigProps {
            settings: Settings {
                sound_volume: 0.5,
                ..Settings::default()
            },
            on_change: Callback::noop()
        },
    );
}
