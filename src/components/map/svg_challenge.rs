use crate::components::map::SCALE;
use crate::components::{BrowserCoordinate, ModelCoordinate};
use konnektoren_core::challenges::ChallengeConfig;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SvgChallengeProps {
    pub challenge: ChallengeConfig,
    pub on_click: Callback<(ChallengeConfig, BrowserCoordinate)>,
    pub class_name: String,
}

#[function_component(SvgChallenge)]
pub fn svg_challenge(props: &SvgChallengeProps) -> Html {
    let model_pos = ModelCoordinate(
        props.challenge.position.unwrap_or_default().0,
        props.challenge.position.unwrap_or_default().1,
    );
    let svg_pos = model_pos.to_svg(SCALE);
    let name = props.challenge.name.clone();

    let on_click = {
        let challenge = props.challenge.clone();
        let callback = props.on_click.clone();
        Callback::from(move |e: MouseEvent| {
            let challenge = challenge.clone();
            let callback = callback.clone();
            e.stop_propagation();
            let (x, y) = (e.offset_x() as f64, e.offset_y() as f64);
            let browser_coord = BrowserCoordinate(x, y);
            callback.emit((challenge, browser_coord));
        })
    };

    html! {
        <>
            <circle
                class={props.class_name.clone()}
                cx={svg_pos.0.to_string()}
                cy={svg_pos.1.to_string()}
                r="3"
                onclick={on_click.clone()}
            />
            <text
                x={svg_pos.0.to_string()}
                y={svg_pos.1.to_string()}
                font-size="3"
                text-anchor="middle"
                alignment-baseline="middle"
                onclick={on_click}
            >
                {name}
            </text>
        </>
    }
}
