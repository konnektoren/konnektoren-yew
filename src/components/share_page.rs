use gloo::timers::callback::Timeout;
use urlencoding::encode;
use yew::prelude::*;
use yew_hooks::use_clipboard;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct SharePageProps {
    #[prop_or_default]
    pub url: Option<String>,
}

#[function_component(SharePageComp)]
pub fn share_page_comp(props: &SharePageProps) -> Html {
    let clipboard_handle = use_clipboard();
    let show_copied_message = use_state(|| false);

    let current_url = web_sys::window()
        .unwrap()
        .location()
        .href()
        .unwrap_or_default();
    let share_url = match props.url.clone() {
        Some(url) => format!("{}", encode(&url)),
        None => current_url,
    };

    let on_share_click = {
        let clipboard_handle = clipboard_handle.clone();
        let data = share_url.clone();
        let show_copied_message = show_copied_message.clone();
        Callback::from(move |_| {
            clipboard_handle.write_text(data.to_string());
            show_copied_message.set(true);
            let show_copied_message = show_copied_message.clone();
            Timeout::new(3000, move || {
                show_copied_message.set(false);
            })
            .forget();
        })
    };

    html! {
        <div class="share-page">
            <input type="text" class="share-page__input" readonly=true value={share_url.clone()} />
            <button onclick={on_share_click} class="btn btn--primary">{ "Share" }</button>
            if *show_copied_message {
                <p class="share-page__message">{"Link copied to clipboard!"}</p>
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
