use crate::components::map::SCALE;
use crate::components::map::bounds::Bounds;
use crate::components::map::svg_challenge::SvgChallenge;
use crate::components::map::svg_path::SvgPath;
use crate::components::{BrowserCoordinate, ModelCoordinate, SvgCoordinate};
use crate::prelude::ChallengeIndex;
use konnektoren_core::challenges::ChallengeConfig;
use konnektoren_core::game::GamePath;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct SvgMapProps {
    #[prop_or(1000)]
    pub width: u32,
    #[prop_or(1000)]
    pub height: u32,
    pub view_box: (SvgCoordinate, SvgCoordinate),
    pub game_path: GamePath,
    pub current_challenge: usize,
    #[prop_or_default]
    pub on_select_challenge: Option<Callback<(Option<ChallengeIndex>, BrowserCoordinate)>>,
    #[prop_or_default]
    pub points: Option<usize>,
    #[prop_or_default]
    pub image_src: Option<String>,
}

#[function_component(SvgMap)]
pub fn svg_map(props: &SvgMapProps) -> Html {
    let view_box = format!(
        "{} {} {} {}",
        props.view_box.0.0,
        props.view_box.0.1,
        props.view_box.1.0 - props.view_box.0.0,
        props.view_box.1.1 - props.view_box.0.1
    );

    let on_map_click = {
        let on_select_challenge = props.on_select_challenge.clone();
        Callback::from(move |e: MouseEvent| {
            let browser_coord = BrowserCoordinate(e.offset_x() as f64, e.offset_y() as f64);
            if let Some(ref callback) = on_select_challenge {
                callback.emit((None, browser_coord));
            }
        })
    };

    let bounds = props.game_path.get_bounds();

    let image_x = bounds.0.to_svg(SCALE).0 - SCALE;
    let image_y = bounds.0.to_svg(SCALE).1 - SCALE;
    let image_width = bounds.1.to_svg(SCALE).0 - bounds.0.to_svg(SCALE).0 + 2 * SCALE;
    let image_height = bounds.1.to_svg(SCALE).1 - bounds.0.to_svg(SCALE).1 + 2 * SCALE;

    let image_src = props
        .game_path
        .map
        .as_ref()
        .map(|map| map.background.clone());

    let image_src: Option<String> = props.image_src.as_ref().or(image_src.as_ref()).cloned();

    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width={props.width.to_string()}
            height={props.height.to_string()}
            viewBox={view_box}
            class="game-map-svg"
            onclick={on_map_click}
        >
            { if let Some(ref image_src) = image_src {
                html! {
                    <image
                    href={image_src.clone()}
                    x={image_x.to_string()}
                    y={image_y.to_string()}
                    width={image_width.to_string()}
                    height={image_height.to_string()}
                    />
                }
            } else {
                html! {}
            }}
            {for props.game_path.challenges.iter().enumerate().map(|(index, _)| {
                render_challenge(props, index)
            })}
        </svg>
    }
}

fn render_challenge(props: &SvgMapProps, index: usize) -> Html {
    let challenge = props.game_path.challenges.get(index).unwrap();
    let position = challenge.position.unwrap_or((0, 0));
    let model_coord = ModelCoordinate(position.0, position.1);
    let next_challenge = props.game_path.challenges.get(index + 1);

    let class_name = format!(
        "{} {}",
        if props.current_challenge == index {
            "selected-circle"
        } else {
            "unselected-circle"
        },
        if let Some(points) = props.points {
            if points >= challenge.unlock_points {
                "unlocked-circle"
            } else {
                "locked-circle"
            }
        } else {
            "unlocked-circle"
        }
    );

    let on_click = {
        let on_select_challenge = props.on_select_challenge.clone();
        Callback::from(
            move |(_challenge, browser_coord): (ChallengeConfig, BrowserCoordinate)| {
                if let Some(ref callback) = on_select_challenge {
                    callback.emit((Some(index), browser_coord));
                }
            },
        )
    };

    html! {
        <>
            {if let Some(next) = next_challenge {
                let next_position = next.position.unwrap_or((0, 0));
                let next_model_coord = ModelCoordinate(next_position.0, next_position.1);
                html! {
                    <SvgPath poly={vec![model_coord, next_model_coord]} />
                }
            } else {
                html! {}
            }}
            <SvgChallenge challenge={challenge.clone()} on_click={on_click} class_name={class_name} />
        </>
    }
}
