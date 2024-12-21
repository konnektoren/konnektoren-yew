use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LogoProps {
    pub img_src: String,
}

#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    html! {
        <div class="logo">
            <img src={props.img_src.clone()} alt="Logo" />
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        Logo,
        LogoProps {
            img_src: "https://konnektoren.help/favicon.png".to_string(),
        },
    );
}
