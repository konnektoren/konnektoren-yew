use crate::components::map::bounds::Bounds;
use crate::components::map::svg_map::SvgMap;
use crate::components::map::utils::Zoom;
use crate::components::map::SCALE;
use crate::components::{BrowserCoordinate, ChallengeIndex, SvgCoordinate};
use konnektoren_core::game::GamePath;
use konnektoren_core::prelude::ChallengeConfig;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct MapComponentProps {
    pub game_path: GamePath,
    pub current_challenge: usize,
    #[prop_or_default]
    pub on_select_challenge: Option<Callback<(Option<ChallengeIndex>, BrowserCoordinate)>>,
    #[prop_or_default]
    pub points: Option<usize>,
    #[prop_or_default]
    pub image_src: Option<String>,
    #[prop_or(1000)]
    pub width: u32,
    #[prop_or(1000)]
    pub height: u32,
}

impl Default for MapComponentProps {
    fn default() -> Self {
        Self {
            game_path: GamePath::default(),
            current_challenge: 0,
            on_select_challenge: None,
            points: None,
            image_src: None,
            width: 1000,
            height: 1000,
        }
    }
}

const MIN_ZOOM: f64 = 0.5;
const MAX_ZOOM: f64 = 10.0;
const DEFAULT_MODEL_WIDTH: u32 = 10;
const DEFAULT_MODEL_HEIGHT: u32 = 10;

#[function_component(MapComponent)]
pub fn map_component(props: &MapComponentProps) -> Html {
    let bounds = props.game_path.get_bounds();
    let svg_bounds = (bounds.0.to_svg(SCALE), bounds.1.to_svg(SCALE));

    let view_box_state = use_state(|| svg_bounds.clone());
    let zoom_level = use_state(|| {
        props
            .game_path
            .get_zoom(DEFAULT_MODEL_WIDTH, DEFAULT_MODEL_HEIGHT)
    });
    let is_dragging = use_state(|| false);
    let last_touch_pos = use_state(|| (0.0, 0.0));
    let view_box_position = use_state(|| svg_bounds.0.clone());

    let handle_wheel =
        handle_wheel_callback(&view_box_state, &zoom_level, &view_box_position, svg_bounds);

    let on_mouse_move = on_mouse_move_callback(
        &is_dragging,
        &last_touch_pos,
        &view_box_position,
        &zoom_level,
        &view_box_state,
        svg_bounds,
    );

    let on_touch_move = on_touch_move_callback(
        &is_dragging,
        &last_touch_pos,
        &view_box_position,
        &zoom_level,
        &view_box_state,
        svg_bounds,
    );

    let on_mouse_down = {
        let is_dragging = is_dragging.clone();
        let last_touch_pos = last_touch_pos.clone();
        Callback::from(move |e: MouseEvent| {
            is_dragging.set(true);
            last_touch_pos.set((e.client_x() as f64, e.client_y() as f64));
        })
    };

    let on_touch_start = {
        let is_dragging = is_dragging.clone();
        let last_touch_pos = last_touch_pos.clone();
        Callback::from(move |e: TouchEvent| {
            let touch = e.touches().get(0).unwrap();
            is_dragging.set(true);
            last_touch_pos.set((touch.client_x() as f64, touch.client_y() as f64));
        })
    };

    let on_mouse_up = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |_| is_dragging.set(false))
    };

    let on_touch_end = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |_| is_dragging.set(false))
    };

    let zoom_in = {
        let zoom_level = zoom_level.clone();
        let view_box_state = view_box_state.clone();
        let view_box_position = view_box_position.clone();
        Callback::from(move |_| {
            adjust_zoom(
                &zoom_level,
                &view_box_state,
                &view_box_position,
                svg_bounds,
                0.1,
            );
        })
    };

    let zoom_out = {
        let zoom_level = zoom_level.clone();
        let view_box_state = view_box_state.clone();
        let view_box_position = view_box_position.clone();
        Callback::from(move |_| {
            adjust_zoom(
                &zoom_level,
                &view_box_state,
                &view_box_position,
                svg_bounds,
                -0.1,
            );
        })
    };

    {
        let view_box_state = view_box_state.clone();
        let game_path = props.game_path.clone();
        use_effect_with(props.game_path.clone(), move |_| {
            let view_box_state = view_box_state.clone();
            let bounds = game_path.get_bounds();
            let svg_bounds = (bounds.0.to_svg(SCALE), bounds.1.to_svg(SCALE));

            if let Some(first_challenge) = game_path.challenges.first() {
                let (center, new_zoom) = calculate_challenge_focus(first_challenge, svg_bounds);

                zoom_level.set(new_zoom);

                let width = ((svg_bounds.1 .0 - svg_bounds.0 .0) as f64 / new_zoom) as i32;
                let height = ((svg_bounds.1 .1 - svg_bounds.0 .1) as f64 / new_zoom) as i32;

                let new_min_x = center.0 - width / 2;
                let new_min_y = center.1 - height / 2;

                let updated_view_box = (
                    SvgCoordinate(new_min_x, new_min_y),
                    SvgCoordinate(new_min_x + width, new_min_y + height),
                );

                view_box_state.set(updated_view_box);
                view_box_position.set(SvgCoordinate(new_min_x, new_min_y));
            }

            || ()
        });
    }

    html! {
        <div class="map"
            onwheel={handle_wheel}
            onmousedown={on_mouse_down}
            onmouseup={on_mouse_up}
            onmousemove={on_mouse_move}
            ontouchstart={on_touch_start}
            ontouchend={on_touch_end}
            ontouchmove={on_touch_move}
        >
            <h2>{&props.game_path.name}</h2>
            <SvgMap
                width={props.width}
                height={props.height}
                view_box={*view_box_state}
                game_path={props.game_path.clone()}
                current_challenge={props.current_challenge}
                on_select_challenge={props.on_select_challenge.clone()}
                points={props.points}
                image_src={props.image_src.clone()}
            />

            <div class="zoom-controls">
                <button onclick={zoom_in}>{"+"}</button>
                <button onclick={zoom_out}>{"-"}</button>
            </div>
        </div>
    }
}

fn calculate_challenge_focus(
    challenge: &ChallengeConfig,
    bounds: (SvgCoordinate, SvgCoordinate),
) -> (SvgCoordinate, f64) {
    let challenge_pos = challenge.position.unwrap_or((0, 0));
    let center = SvgCoordinate(challenge_pos.0 * SCALE, challenge_pos.1 * SCALE);

    let width = (bounds.1 .0 - bounds.0 .0) as f64;
    let height = (bounds.1 .1 - bounds.0 .1) as f64;
    let zoom = (width.min(height) / (5.0 * SCALE as f64))
        .max(MIN_ZOOM)
        .min(MAX_ZOOM);

    (center, zoom)
}

fn on_touch_move_callback(
    is_dragging: &UseStateHandle<bool>,
    last_touch_pos: &UseStateHandle<(f64, f64)>,
    view_box_position: &UseStateHandle<SvgCoordinate>,
    zoom_level: &UseStateHandle<f64>,
    view_box: &UseStateHandle<(SvgCoordinate, SvgCoordinate)>,
    bounds: (SvgCoordinate, SvgCoordinate),
) -> Callback<TouchEvent> {
    let is_dragging = is_dragging.clone();
    let last_touch_pos = last_touch_pos.clone();
    let view_box_position = view_box_position.clone();
    let zoom_level = zoom_level.clone();
    let view_box = view_box.clone();

    Callback::from(move |e: TouchEvent| {
        if *is_dragging {
            let touch = e.touches().get(0).unwrap();
            let (dx, dy) =
                calculate_mouse_delta(touch.client_x(), touch.client_y(), &*last_touch_pos);
            let (dx, dy) = (dx.max(-1.0).min(1.0), dy.max(-1.0).min(1.0));

            if dx.is_nan() || dy.is_nan() {
                log::error!("Invalid touch movement delta: dx={}, dy={}", dx, dy);
                return;
            }

            last_touch_pos.set((touch.client_x() as f64, touch.client_y() as f64));

            let (view_box_width, view_box_height) = calculate_view_box_size(bounds, *zoom_level);
            if view_box_width > 0 && view_box_height > 0 {
                let (new_view_box_x, new_view_box_y) = calculate_new_view_box_position(
                    &view_box_position,
                    dx,
                    dy,
                    view_box_width,
                    view_box_height,
                    bounds,
                );

                update_view_box(
                    &view_box_position,
                    &view_box,
                    new_view_box_x,
                    new_view_box_y,
                    view_box_width,
                    view_box_height,
                );
            }
        }
    })
}

fn adjust_zoom(
    zoom_level: &UseStateHandle<f64>,
    view_box_state: &UseStateHandle<(SvgCoordinate, SvgCoordinate)>,
    view_box_position: &UseStateHandle<SvgCoordinate>,
    bounds: (SvgCoordinate, SvgCoordinate),
    zoom_delta: f64,
) {
    let new_zoom = (*(*zoom_level) + zoom_delta).clamp(MIN_ZOOM, MAX_ZOOM);
    zoom_level.set(new_zoom);

    let width = ((bounds.1 .0 - bounds.0 .0) as f64 / new_zoom).max(1.0);
    let height = ((bounds.1 .1 - bounds.0 .1) as f64 / new_zoom).max(1.0);

    let new_min_x = view_box_position
        .0
        .min(bounds.1 .0 - width as i32)
        .max(bounds.0 .0);
    let new_min_y = view_box_position
        .1
        .min(bounds.1 .1 - height as i32)
        .max(bounds.0 .1);

    let updated_view_box: (SvgCoordinate, SvgCoordinate) = (
        SvgCoordinate(new_min_x, new_min_y),
        SvgCoordinate(new_min_x + width as i32, new_min_y + height as i32),
    );
    view_box_state.set(updated_view_box);
}

fn handle_wheel_callback(
    view_box_state: &UseStateHandle<(SvgCoordinate, SvgCoordinate)>,
    zoom_level: &UseStateHandle<f64>,
    view_box_position: &UseStateHandle<SvgCoordinate>,
    bounds: (SvgCoordinate, SvgCoordinate),
) -> Callback<WheelEvent> {
    let view_box = view_box_state.clone();
    let zoom_level = zoom_level.clone();
    let view_box_position = view_box_position.clone();

    Callback::from(move |e: WheelEvent| {
        e.prevent_default();
        let zoom_delta = e.delta_y().signum() * 0.1;
        adjust_zoom(
            &zoom_level,
            &view_box,
            &view_box_position,
            bounds,
            zoom_delta,
        );
    })
}

fn on_mouse_move_callback(
    is_dragging: &UseStateHandle<bool>,
    last_mouse_pos: &UseStateHandle<(f64, f64)>,
    view_box_position: &UseStateHandle<SvgCoordinate>,
    zoom_level: &UseStateHandle<f64>,
    view_box: &UseStateHandle<(SvgCoordinate, SvgCoordinate)>,
    bounds: (SvgCoordinate, SvgCoordinate),
) -> Callback<MouseEvent> {
    let is_dragging = is_dragging.clone();
    let last_mouse_pos = last_mouse_pos.clone();
    let view_box_position = view_box_position.clone();
    let zoom_level = zoom_level.clone();
    let view_box = view_box.clone();

    Callback::from(move |e: MouseEvent| {
        if *is_dragging {
            let (dx, dy) = calculate_mouse_delta(e.client_x(), e.client_y(), &*last_mouse_pos);
            let (dx, dy) = (dx.max(-1.0).min(1.0), dy.max(-1.0).min(1.0));

            if dx.is_nan() || dy.is_nan() {
                log::error!("Invalid mouse movement delta: dx={}, dy={}", dx, dy);
                return;
            }

            last_mouse_pos.set((e.client_x() as f64, e.client_y() as f64));

            let (view_box_width, view_box_height) = calculate_view_box_size(bounds, *zoom_level);
            if view_box_width > 0 && view_box_height > 0 {
                let (new_view_box_x, new_view_box_y) = calculate_new_view_box_position(
                    &view_box_position,
                    dx,
                    dy,
                    view_box_width,
                    view_box_height,
                    bounds,
                );

                update_view_box(
                    &view_box_position,
                    &view_box,
                    new_view_box_x,
                    new_view_box_y,
                    view_box_width,
                    view_box_height,
                );
            }
        }
    })
}

fn calculate_mouse_delta(client_x: i32, client_y: i32, last_pos: &(f64, f64)) -> (f64, f64) {
    let dx = client_x as f64 - last_pos.0;
    let dy = client_y as f64 - last_pos.1;
    (dx, dy)
}

fn calculate_view_box_size(bounds: (SvgCoordinate, SvgCoordinate), zoom_level: f64) -> (i32, i32) {
    let view_box_width = ((bounds.1 .0 - bounds.0 .0) as f64 / zoom_level) as i32;
    let view_box_height = ((bounds.1 .1 - bounds.0 .1) as f64 / zoom_level) as i32;
    (view_box_width, view_box_height)
}

fn calculate_new_view_box_position(
    view_box_position: &UseStateHandle<SvgCoordinate>,
    dx: f64,
    dy: f64,
    view_box_width: i32,
    view_box_height: i32,
    bounds: (SvgCoordinate, SvgCoordinate),
) -> (i32, i32) {
    let new_view_box_x = view_box_position.0 - dx as i32;
    let new_view_box_y = view_box_position.1 - dy as i32;

    let new_view_box_x = clamp_position(
        new_view_box_x,
        bounds.0 .0 - SCALE,
        bounds.1 .0 - view_box_width + SCALE,
        "x",
    );
    let new_view_box_y = clamp_position(
        new_view_box_y,
        bounds.0 .1 - SCALE,
        bounds.1 .1 - view_box_height + SCALE,
        "y",
    );

    (new_view_box_x, new_view_box_y)
}

fn clamp_position(value: i32, min: i32, max: i32, axis: &str) -> i32 {
    if min <= max {
        value.clamp(min, max)
    } else {
        log::error!(
            "Invalid clamp range on {}-axis: min ({}) > max ({})",
            axis,
            min,
            max
        );
        min
    }
}

fn update_view_box(
    view_box_position: &UseStateHandle<SvgCoordinate>,
    view_box: &UseStateHandle<(SvgCoordinate, SvgCoordinate)>,
    new_x: i32,
    new_y: i32,
    view_box_width: i32,
    view_box_height: i32,
) {
    view_box_position.set(SvgCoordinate(new_x, new_y));

    let updated_view_box = (
        SvgCoordinate(new_x, new_y),
        SvgCoordinate(new_x + view_box_width, new_y + view_box_height),
    );
    view_box.set(updated_view_box);
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew::callback;
    use yew_preview::prelude::*;

    fn props() -> MapComponentProps {
        let callback = callback::Callback::from(
            move |(challenge_index, coordinate): (Option<ChallengeIndex>, BrowserCoordinate)| {
                if let Some(challenge_index) = challenge_index {
                    log::info!("Challenge selected: {}", challenge_index);
                } else {
                    log::info!("Challenge deselected {} {}", coordinate.0, coordinate.1);
                }
            },
        );
        let mut props = MapComponentProps::default();
        props.current_challenge = 1;
        props.points = Some(100);
        props.on_select_challenge = Some(callback);
        props.image_src = Some("https://picsum.photos/800".to_string());
        props.width = 800;
        props.height = 800;
        props
    }

    yew_preview::create_preview!(
        MapComponent,
        MapComponentProps::default(),
        ("Background", props())
    );
}
