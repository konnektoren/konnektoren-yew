use crate::model::TargetRect;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlaybackHighlightProps {
    pub rect: TargetRect,
}

/// Visually marks a target without intercepting pointer input.
#[function_component(PlaybackHighlight)]
pub fn playback_highlight(props: &PlaybackHighlightProps) -> Html {
    let rect = props.rect;
    let style = format!(
        "left:{}px;top:{}px;width:{}px;height:{}px",
        rect.x, rect.y, rect.width, rect.height
    );
    html! { <div class="playback-highlight" {style} aria-hidden="true" /> }
}

/// Highlights the first element matching a CSS selector, following layout changes.
#[derive(Properties, PartialEq)]
pub struct PlaybackElementHighlightProps {
    pub selector: AttrValue,
}

#[function_component(PlaybackElementHighlight)]
pub fn playback_element_highlight(props: &PlaybackElementHighlightProps) -> Html {
    let rect = use_state_eq(|| None::<TargetRect>);

    #[cfg(feature = "csr")]
    {
        let rect = rect.clone();
        use_effect_with(props.selector.clone(), move |selector| {
            let selector = selector.clone();
            let update = move || {
                let next = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.query_selector(&selector).ok().flatten())
                    .map(|element| {
                        let bounds = element.get_bounding_client_rect();
                        TargetRect {
                            x: bounds.x(),
                            y: bounds.y(),
                            width: bounds.width(),
                            height: bounds.height(),
                        }
                    });
                rect.set(next);
            };
            update();
            let timer = gloo_timers::callback::Interval::new(100, update);
            move || drop(timer)
        });
    }
    #[cfg(not(feature = "csr"))]
    let _ = props;

    rect.as_ref().map_or_else(Html::default, |rect| {
        html! {
            <PlaybackHighlight rect={*rect} />
        }
    })
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        PlaybackHighlight,
        PlaybackHighlightProps {
            rect: TargetRect {
                x: 40.0,
                y: 100.0,
                width: 220.0,
                height: 64.0
            },
        },
        (
            "Small target",
            PlaybackHighlightProps {
                rect: TargetRect {
                    x: 80.0,
                    y: 180.0,
                    width: 80.0,
                    height: 40.0,
                },
            }
        ),
        (
            "Wide target",
            PlaybackHighlightProps {
                rect: TargetRect {
                    x: 40.0,
                    y: 100.0,
                    width: 360.0,
                    height: 120.0,
                },
            }
        ),
    );

    yew_preview::create_preview!(
        PlaybackElementHighlight,
        PlaybackElementHighlightProps {
            selector: "#component-preview-title".into()
        },
        (
            "Component heading",
            PlaybackElementHighlightProps {
                selector: ".yew-preview-component > h2".into(),
            }
        ),
    );
}
