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

/// Highlights every element matching a CSS selector, following layout changes.
#[derive(Properties, PartialEq)]
pub struct PlaybackElementHighlightProps {
    pub selector: AttrValue,
}

#[function_component(PlaybackElementHighlight)]
pub fn playback_element_highlight(props: &PlaybackElementHighlightProps) -> Html {
    let rects = use_state_eq(Vec::<TargetRect>::new);

    #[cfg(feature = "csr")]
    {
        use wasm_bindgen::JsCast;

        let rects = rects.clone();
        use_effect_with(props.selector.clone(), move |selector| {
            let selector = selector.clone();
            let update = move || {
                let next = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.query_selector_all(&selector).ok())
                    .map(|elements| {
                        (0..elements.length())
                            .filter_map(|index| elements.item(index))
                            .filter_map(|node| node.dyn_into::<web_sys::Element>().ok())
                            .map(|element| {
                                let bounds = element.get_bounding_client_rect();
                                TargetRect {
                                    x: bounds.x(),
                                    y: bounds.y(),
                                    width: bounds.width(),
                                    height: bounds.height(),
                                }
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                rects.set(next);
            };
            update();
            let timer = gloo_timers::callback::Interval::new(100, update);
            move || drop(timer)
        });
    }
    #[cfg(not(feature = "csr"))]
    let _ = props;

    html! { <>{for rects.iter().map(|rect| html! { <PlaybackHighlight rect={*rect} /> })}</> }
}

#[cfg(feature = "yew-preview")]
#[derive(Properties, PartialEq)]
pub struct PlaybackHighlightDemoProps {
    pub selector: AttrValue,
}

/// Sample content for previewing element highlights.
#[cfg(feature = "yew-preview")]
#[function_component(PlaybackHighlightDemo)]
pub fn playback_highlight_demo(props: &PlaybackHighlightDemoProps) -> Html {
    html! {
        <div class="playback-highlight-demo">
            <header class="playback-highlight-demo__header"><h3>{"Example page"}</h3></header>
            <p class="playback-highlight-demo__body">{"Explore the different parts of this page."}</p>
            <ul class="playback-highlight-demo__list">
                <li>{"First item"}</li>
                <li>{"Second item"}</li>
                <li>{"Third item"}</li>
            </ul>
            <PlaybackElementHighlight selector={props.selector.clone()} />
        </div>
    }
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
        (
            "Both headings",
            PlaybackElementHighlightProps {
                selector: "#component-preview-title, .yew-preview-component > h2".into(),
            }
        ),
        (
            "Sidebar headings",
            PlaybackElementHighlightProps {
                selector: ".yew-preview-sidebar-section > h2".into(),
            }
        ),
    );

    yew_preview::create_preview!(
        PlaybackHighlightDemo,
        PlaybackHighlightDemoProps {
            selector: ".playback-highlight-demo__header".into()
        },
        (
            "Body",
            PlaybackHighlightDemoProps {
                selector: ".playback-highlight-demo__body".into()
            }
        ),
        (
            "List",
            PlaybackHighlightDemoProps {
                selector: ".playback-highlight-demo__list".into()
            }
        ),
        (
            "List items",
            PlaybackHighlightDemoProps {
                selector: ".playback-highlight-demo__list > li".into()
            }
        ),
    );
}
