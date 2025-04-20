use crate::components::MusicConfig;
use crate::components::settings::sound_config::SoundConfig;
use crate::i18n::use_i18n;
use crate::model::Settings;
use crate::prelude::use_settings;
use yew::prelude::*;

#[function_component(SettingsComponent)]
pub fn settings_component() -> Html {
    let i18n = use_i18n();
    let settings = use_settings();
    let initial_settings = use_state(|| (*settings).clone());

    let on_change = {
        let settings = settings.clone();
        Callback::from(move |new_settings: Settings| {
            settings.set(new_settings.clone());
        })
    };

    let on_save = {
        let settings = settings.clone();
        let initial_settings = initial_settings.clone();
        Callback::from(move |_| {
            initial_settings.set((*settings).clone());
        })
    };

    let has_changes = {
        let settings = (*settings).clone();
        let new_settings = (*initial_settings).clone();
        move || settings != new_settings
    };

    html! {
        <div class="settings">
            <h2>{ i18n.t("Settings") }</h2>

            <MusicConfig settings={(*settings).clone()} on_change={on_change.clone()} />
            <SoundConfig settings={(*settings).clone()} on_change={on_change.clone()} />
            <button onclick={on_save} disabled={!has_changes()}>{ i18n.t("Save") }</button>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(SettingsComponent, (),);
}
