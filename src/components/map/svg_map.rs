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

    // Image is fixed in SVG coordinate space so it zooms and pans with the challenges.
    // When the level declares map dimensions, the image covers exactly that area (origin 0,0).
    // Otherwise it falls back to the challenge-position bounds.
    let (image_x, image_y, image_width, image_height) = if let Some(map) = &props.game_path.map {
        (0, 0, map.width as i32 * SCALE, map.height as i32 * SCALE)
    } else {
        let bounds = props.game_path.get_bounds();
        (
            bounds.0.to_svg(SCALE).0,
            bounds.0.to_svg(SCALE).1,
            (bounds.1.0 - bounds.0.0) * SCALE,
            (bounds.1.1 - bounds.0.1) * SCALE,
        )
    };

    let bounds = props.game_path.get_bounds();

    let image_src = props
        .game_path
        .map
        .as_ref()
        .map(|map| map.background.clone())
        .filter(|s| !s.is_empty());
    let image_src: Option<String> = props.image_src.as_ref().or(image_src.as_ref()).cloned();

    // Compute node radius from the challenge-position span (not map bounds)
    let node_radius = compute_node_radius(&props.game_path);
    let stroke_width = (node_radius * 0.35).max(0.5);

    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width={props.width.to_string()}
            height={props.height.to_string()}
            viewBox={view_box}
            class="game-map-svg"
            onclick={on_map_click}
        >
            { if let Some(ref src) = image_src {
                html! {
                    <image
                        href={src.clone()}
                        x={image_x.to_string()}
                        y={image_y.to_string()}
                        width={image_width.to_string()}
                        height={image_height.to_string()}
                        preserveAspectRatio="xMidYMid meet"
                    />
                }
            } else {
                html! {}
            }}
            {for props.game_path.challenges.iter().enumerate().map(|(index, _)| {
                render_challenge(props, index, node_radius, stroke_width)
            })}
        </svg>
    }
}

/// Compute a node radius proportional to the spread of challenge positions.
fn compute_node_radius(game_path: &GamePath) -> f64 {
    let positions: Vec<(i32, i32)> = game_path
        .challenges
        .iter()
        .filter_map(|c| c.position)
        .collect();

    if positions.is_empty() {
        return 5.0;
    }

    let x_min = positions.iter().map(|p| p.0).min().unwrap_or(0);
    let x_max = positions.iter().map(|p| p.0).max().unwrap_or(1);
    let y_min = positions.iter().map(|p| p.1).min().unwrap_or(0);
    let y_max = positions.iter().map(|p| p.1).max().unwrap_or(1);

    let span_w = ((x_max - x_min) * SCALE).max(SCALE) as f64;
    let span_h = ((y_max - y_min) * SCALE).max(SCALE) as f64;
    let span = span_w.min(span_h);

    (span / 15.0).clamp(2.0, 30.0)
}

fn render_challenge(
    props: &SvgMapProps,
    index: usize,
    node_radius: f64,
    stroke_width: f64,
) -> Html {
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
                    <SvgPath poly={vec![model_coord, next_model_coord]} stroke_width={stroke_width} />
                }
            } else {
                html! {}
            }}
            <SvgChallenge
                challenge={challenge.clone()}
                on_click={on_click}
                class_name={class_name}
                radius={node_radius}
            />
        </>
    }
}
