use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TranslateProps {
    pub text: String,
}

#[function_component(TranslateComponent)]
pub fn translate(props: &TranslateProps) -> Html {
    let google_translate_url = format!(
        "https://translate.google.com/?sl=auto&tl=en&text={}",
        props.text
    );
    html! {
        <div class="translate-button">
            <a href={google_translate_url} target="_blank" rel="noopener noreferrer">
                { "Translate" }
            </a>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        TranslateComponent,
        TranslateProps {
            text: "Hallo Welt".to_string()
        },
        (
            "Hello World",
            TranslateProps {
                text: "Hello World".to_string()
            }
        )
    );
}
