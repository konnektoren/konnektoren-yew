use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlaybackCursorProps {
    /// Position as a percentage of the viewport width and height.
    pub x: f64,
    pub y: f64,
    #[prop_or_default]
    pub image: Option<AttrValue>,
}

/// A decorative pointer that moves between playback targets.
#[function_component(PlaybackCursor)]
pub fn playback_cursor(props: &PlaybackCursorProps) -> Html {
    let style = format!(
        "left:clamp(0px, {}%, calc(100vw - 24px));top:clamp(0px, {}%, calc(100vh - 32px));",
        props.x, props.y
    );
    let class = classes!(
        "playback-cursor",
        (props.x > 50.0).then_some("playback-cursor--left"),
        (props.y > 50.0).then_some("playback-cursor--above"),
    );
    html! {
        <div {class} {style} aria-hidden="true">
            <svg class="playback-cursor__pointer" viewBox="0 0 24 30">
                <path d="M2 2v21l5.5-5 4.5 9 4-2-4.5-9H20z" />
            </svg>
            if let Some(image) = &props.image {
                <img class="playback-cursor__image" src={image.clone()} alt="" />
            }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        PlaybackCursor,
        PlaybackCursorProps { x: 40.0, y: 40.0, image: None },
        ("With image", PlaybackCursorProps {
            x: 70.0, y: 70.0,
            image: Some("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 48 48'%3E%3Ccircle cx='24' cy='24' r='22' fill='%23f97316'/%3E%3C/svg%3E".into()),
        }),
    );
}
