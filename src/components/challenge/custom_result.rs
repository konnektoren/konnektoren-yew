use crate::components::custom::fetch_content;
use crate::i18n::{I18nLoader, I18nYmlLoader, SelectedLanguage};
use konnektoren_core::challenges::{Custom, CustomChallengeResult};
use konnektoren_core::konnektoren_js::KonnektorenJs;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CustomResultComponentProps {
    pub challenge: Custom,
    pub result: CustomChallengeResult,
}

#[function_component(CustomResultComponent)]
pub fn custom_result(props: &CustomResultComponentProps) -> Html {
    if props.challenge.results_html.is_none() {
        return html! {
            <div class="custom-result">
                <p>{"No results page available for this challenge."}</p>
            </div>
        };
    }

    let html_content = use_state(|| "".to_string());
    let css_content = use_state(|| "".to_string());
    let js_content = use_state(|| "".to_string());
    let i18n_content = use_state(|| "".to_string());
    let loading = use_state(|| true);

    let konnektoren_js = use_mut_ref(|| {
        let window = web_sys::window().expect("no global `window` exists");
        KonnektorenJs::new(&window)
    });

    // Effect to fetch content when the challenge changes
    {
        let html_content = html_content.clone();
        let css_content = css_content.clone();
        let js_content = js_content.clone();
        let i18n_content = i18n_content.clone();
        let challenge = props.challenge.clone();
        let loading = loading.clone();

        use_effect_with(props.challenge.id.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(results_html) = &challenge.results_html {
                    fetch_content(results_html, html_content).await;
                }
                fetch_content(&challenge.css, css_content).await;
                fetch_content(&challenge.js, js_content).await;
                if let Some(challenge_i18n) = &challenge.i18n {
                    fetch_content(challenge_i18n, i18n_content).await;
                }
                loading.set(false);
            });
            || ()
        });
    }

    // Effect to set up i18n data
    {
        let konnektoren_js = konnektoren_js.clone();
        let i18n_content = i18n_content.clone();

        use_effect_with((*i18n_content).clone(), move |content| {
            if !content.is_empty() {
                let language = SelectedLanguage::default().get();
                let loader = I18nYmlLoader::new(content);
                let translations = loader.get(&language.code()).unwrap_or_default();
                konnektoren_js.borrow_mut().set_i18n_data(translations);
            }
            || ()
        });
    }

    // Effect to set challenge data, result data, and execute JS
    {
        let konnektoren_js = konnektoren_js.clone();
        let challenge = props.challenge.clone();
        let result = props.result.clone();
        let js_code = (*js_content).clone();
        let loading = loading.clone();

        use_effect_with(((*loading), js_code.clone()), move |_| {
            if !*loading {
                let js = konnektoren_js.borrow_mut();
                js.set_challenge_data(challenge);
                js.set_result_data(result);
                js.execute_js(&js_code);
            }
            || ()
        });
    }

    let parsed_html = Html::from_html_unchecked(AttrValue::from((*html_content).clone()));

    html! {
        <div class="custom-result">
            <style>
                {(*css_content).clone()}
            </style>
            {parsed_html}
        </div>
    }
}
