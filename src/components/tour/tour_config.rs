use crate::i18n::use_i18n;
use crate::providers::use_settings;
use yew::prelude::*;

#[function_component(TourConfig)]
pub fn tour_config() -> Html {
    let i18n = use_i18n();
    let settings = use_settings();

    let on_toggle = {
        let settings = settings.clone();
        Callback::from(move |_| {
            let mut new_settings = (*settings).clone();
            new_settings.show_helpers = !new_settings.show_helpers;
            settings.set(new_settings);
        })
    };

    html! {
        <div class="tour-config">
            <label>
                <input
                    type="checkbox"
                    checked={settings.show_helpers}
                    onclick={on_toggle}
                />
                { i18n.t("Show Tour Button") }
            </label>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(TourConfig, (),);
}
