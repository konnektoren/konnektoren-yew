use crate::i18n::{flag, language_name, use_i18n, use_selected_language, LANGUAGES};
use crate::providers::use_settings;
use yew::prelude::*;

#[function_component(SelectLanguage)]
pub fn select_language() -> Html {
    let i18n = use_i18n();
    let selected_language = use_selected_language();
    let settings = use_settings();

    let on_select_change = {
        let selected_language = selected_language.clone();
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            #[cfg(feature = "csr")]
            {
                use web_sys::HtmlSelectElement;
                let mut selected_language = selected_language.clone();
                let settings = settings.clone();
                if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                    log::info!("Selected language: {:?}", select.value());
                    let mut new_settings = (*settings).clone();
                    let value = select.value();
                    new_settings.language = value.clone();
                    selected_language.set(&value.clone());
                    settings.set(new_settings);
                }
            }
        })
    };

    html! {
        <div class="select-language">
            <p>
                { i18n.t("Please select a language from the dropdown.") }
                <select onchange={on_select_change} value={selected_language.get().code()}>
                    <option value="" selected={selected_language.get().code().is_empty()} disabled=true>{ i18n.t("Select Language") }</option>
                    { for LANGUAGES.iter().map(|&lang| html! {
                        <option value={lang} selected={*lang == *selected_language.get().code()}>{format!("{} {}", flag(lang), language_name(lang))}</option>
                    })}
                </select>
            </p>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(SelectLanguage, (),);
}
