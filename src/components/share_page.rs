use crate::i18n::use_i18n;
use urlencoding::encode;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct SharePageProps {
    #[prop_or_default]
    pub url: Option<String>,
}

#[function_component(SharePageComp)]
pub fn share_page_comp(props: &SharePageProps) -> Html {
    let i18n = use_i18n();
    #[cfg(feature = "csr")]
    let clipboard_handle = yew_hooks::use_clipboard();
    let show_copied_message = use_state(|| false);

    let current_url = {
        #[cfg(feature = "csr")]
        {
            web_sys::window()
                .unwrap()
                .location()
                .href()
                .unwrap_or_default()
        }
        #[cfg(not(feature = "csr"))]
        {
            String::new() // Provide a default value for SSR
        }
    };

    let share_url = match props.url.clone() {
        Some(url) => format!("{}", encode(&url)),
        None => current_url,
    };

    let on_share_click = {
        #[cfg(feature = "csr")]
        let clipboard_handle = clipboard_handle.clone();
        let show_copied_message = show_copied_message.clone();
        let share_url = share_url.clone();
        Callback::from(move |_| {
            #[cfg(feature = "csr")]
            {
                let clipboard_handle = clipboard_handle.clone();
                let data = share_url.clone();
                let show_copied_message = show_copied_message.clone();

                clipboard_handle.write_text(data.to_string());
                show_copied_message.set(true);
                let show_copied_message = show_copied_message.clone();
                gloo::timers::callback::Timeout::new(3000, move || {
                    show_copied_message.set(false);
                })
                .forget();
            }
        })
    };

    html! {
        <div class="share-page">
            <input type="text" class="share-page__input" readonly=true value={share_url.clone()} />
            <button onclick={on_share_click} class="btn btn--primary">{ i18n.t("Share") }</button>
            if *show_copied_message {
                <p class="share-page__message">{ i18n.t("Link copied to clipboard!") }</p>
            }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        SharePageComp,
        SharePageProps::default(),
        (
            "example",
            SharePageProps {
                url: Some("https://example.com".to_string()),
            }
        )
    );
}
