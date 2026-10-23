use std::time::Duration;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlinkAnimationProps {
    pub target_id: String,
    #[prop_or_default]
    pub duration: Option<Duration>,
    pub color: String,
    #[prop_or_default]
    pub class_name: Option<String>,
}

#[function_component(BlinkAnimation)]
pub fn blink_animation(props: &BlinkAnimationProps) -> Html {
    let target_id = props.target_id.clone();
    let duration = props.duration.unwrap_or(Duration::from_secs(2)).as_millis() as f64;
    let color = props.color.clone();

    let class_name = props
        .class_name
        .as_ref()
        .unwrap_or(&format!("blink-animation-{}", target_id))
        .to_string();

    let style = format!(
        "@keyframes {} {{
            0% {{ background-color: transparent; }}
            50% {{ background-color: {}; }}
            100% {{ background-color: transparent; }}
        }}

        .{} {{
            animation: {} {}ms linear;
        }}",
        class_name, color, class_name, class_name, duration
    );

    #[cfg(feature = "csr")]
    {
        use gloo::timers::future::TimeoutFuture;
        use wasm_bindgen::JsCast;
        use web_sys::{HtmlElement, window};

        use_effect_with(
            (target_id.clone(), duration as u32, color.clone()),
            move |_| {
                if let Some(document) = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id(&target_id)
                {
                    let element = document.dyn_into::<HtmlElement>().unwrap();
                    // Remove first and force a reflow so the animation restarts on re-run.
                    element.class_list().remove_1(&class_name).unwrap();
                    let _ = element.offset_width();
                    element.class_list().add_1(&class_name).unwrap();

                    let cloned_element = element.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        TimeoutFuture::new((duration) as u32).await;
                        cloned_element.class_list().remove_1(&class_name).unwrap();
                    });
                }
                || ()
            },
        );
    }

    html! {
        <style>{style}</style>
    }
}

#[cfg(feature = "yew-preview")]
pub mod preview {
    use super::*;
    use yew_preview::prelude::*;

    #[derive(Properties, PartialEq)]
    pub struct BlinkAnimationDemoProps {
        pub duration_secs: u64,
        pub color: String,
    }

    /// Renders the element that gets blinked, since the animation only targets existing ids.
    #[function_component(BlinkAnimationDemo)]
    pub fn blink_animation_demo(props: &BlinkAnimationDemoProps) -> Html {
        html! {
            <>
                <div id="blink-animation-demo" style="padding: 2rem; border: 1px solid #ccc;">
                    { "Blink target" }
                </div>
                <BlinkAnimation
                    target_id="blink-animation-demo"
                    duration={Duration::from_secs(props.duration_secs)}
                    color={props.color.clone()}
                />
            </>
        }
    }

    yew_preview::create_preview!(
        BlinkAnimationDemo,
        BlinkAnimationDemoProps {
            duration_secs: 2,
            color: "red".to_string(),
        },
        (
            "1 second",
            BlinkAnimationDemoProps {
                duration_secs: 1,
                color: "red".to_string(),
            }
        ),
        (
            "blue",
            BlinkAnimationDemoProps {
                duration_secs: 2,
                color: "blue".to_string(),
            }
        )
    );
}
