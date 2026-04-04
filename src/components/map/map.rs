use crate::components::map::SCALE;
use crate::components::map::bounds::Bounds;
use crate::components::map::svg_map::SvgMap;
use crate::components::{BrowserCoordinate, ChallengeIndex, SvgCoordinate};
use konnektoren_core::game::GamePath;
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

#[function_component(MapComponent)]
pub fn map_component(props: &MapComponentProps) -> Html {
    let bounds = props.game_path.get_bounds();
    let svg_bounds = (bounds.0.to_svg(SCALE), bounds.1.to_svg(SCALE));

    let view_box_state = use_state(|| svg_bounds);
    let zoom_level = use_state(|| 1.0_f64);
    let is_dragging = use_state(|| false);
    let last_touch_pos = use_state(|| (0.0, 0.0));
    let view_box_position = use_state(|| svg_bounds.0);

    let handle_wheel =
        handle_wheel_callback(&view_box_state, &zoom_level, &view_box_position, svg_bounds);

    let display_size = (props.width, props.height);

    let on_mouse_move = on_mouse_move_callback(
        &is_dragging,
        &last_touch_pos,
        &view_box_position,
        &zoom_level,
        &view_box_state,
        svg_bounds,
        display_size,
    );

    let on_touch_move = on_touch_move_callback(
        &is_dragging,
        &last_touch_pos,
        &view_box_position,
        &zoom_level,
        &view_box_state,
        svg_bounds,
        display_size,
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
            #[cfg(feature = "csr")]
            {
                use wasm_bindgen::JsCast;
                use web_sys::TouchEvent;
                if let Ok(event) = e.dyn_into::<TouchEvent>() {
                    if let Some(touch) = event.touches().get(0) {
                        is_dragging.set(true);
                        last_touch_pos.set((touch.client_x() as f64, touch.client_y() as f64));
                    }
                }
            }
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
            // Start with the full level visible so every challenge is reachable regardless
            // of display aspect ratio.
            let bounds = game_path.get_bounds();
            let svg_bounds = (bounds.0.to_svg(SCALE), bounds.1.to_svg(SCALE));
            zoom_level.set(1.0);
            view_box_state.set(svg_bounds);
            view_box_position.set(svg_bounds.0);
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

/// Pure function: given the full-level SVG bounds, the current viewBox size, and a pan
/// delta (already scaled from browser pixels to SVG units), return the new clamped
/// top-left corner of the viewBox.
///
/// The clamp guarantees every part of the level is reachable:
///   min_corner = bounds.min - SCALE  (one node-radius of over-scroll)
///   max_corner = bounds.max - vb_size + SCALE
///
/// When vb_size > span the max < min; in that case the full level is already in
/// view and position is pinned to bounds.min - SCALE.
pub(crate) fn clamp_view_box_position(
    current: SvgCoordinate,
    delta: (f64, f64),
    vb_size: (i32, i32),
    bounds: (SvgCoordinate, SvgCoordinate),
) -> SvgCoordinate {
    let raw_x = current.0 - delta.0 as i32;
    let raw_y = current.1 - delta.1 as i32;
    SvgCoordinate(
        clamp_position(raw_x, bounds.0.0 - SCALE, bounds.1.0 - vb_size.0 + SCALE, "x"),
        clamp_position(raw_y, bounds.0.1 - SCALE, bounds.1.1 - vb_size.1 + SCALE, "y"),
    )
}

fn on_touch_move_callback(
    is_dragging: &UseStateHandle<bool>,
    last_touch_pos: &UseStateHandle<(f64, f64)>,
    view_box_position: &UseStateHandle<SvgCoordinate>,
    zoom_level: &UseStateHandle<f64>,
    view_box: &UseStateHandle<(SvgCoordinate, SvgCoordinate)>,
    bounds: (SvgCoordinate, SvgCoordinate),
    display_size: (u32, u32),
) -> Callback<TouchEvent> {
    let is_dragging = is_dragging.clone();
    let last_touch_pos = last_touch_pos.clone();
    let view_box_position = view_box_position.clone();
    let zoom_level = zoom_level.clone();
    let view_box = view_box.clone();

    Callback::from(move |e: TouchEvent| {
        #[cfg(feature = "csr")]
        {
            use wasm_bindgen::JsCast;
            use web_sys::TouchEvent;

            if *is_dragging {
                if let Ok(event) = e.dyn_into::<TouchEvent>() {
                    if let Some(touch) = event.touches().get(0) {
                        let (dx, dy) = calculate_mouse_delta(
                            touch.client_x(),
                            touch.client_y(),
                            &last_touch_pos,
                        );

                        if dx.is_nan() || dy.is_nan() {
                            log::error!("Invalid touch movement delta: dx={}, dy={}", dx, dy);
                            return;
                        }

                        last_touch_pos.set((touch.client_x() as f64, touch.client_y() as f64));

                        let (view_box_width, view_box_height) =
                            calculate_view_box_size(bounds, *zoom_level);
                        if view_box_width > 0 && view_box_height > 0 {
                            let scale_x = view_box_width as f64 / display_size.0 as f64;
                            let scale_y = view_box_height as f64 / display_size.1 as f64;
                            let (new_view_box_x, new_view_box_y) = calculate_new_view_box_position(
                                &view_box_position,
                                dx * scale_x,
                                dy * scale_y,
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
                }
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

    let width = ((bounds.1.0 - bounds.0.0) as f64 / new_zoom).max(1.0);
    let height = ((bounds.1.1 - bounds.0.1) as f64 / new_zoom).max(1.0);

    let new_min_x = view_box_position
        .0
        .min(bounds.1.0 - width as i32)
        .max(bounds.0.0);
    let new_min_y = view_box_position
        .1
        .min(bounds.1.1 - height as i32)
        .max(bounds.0.1);

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
    display_size: (u32, u32),
) -> Callback<MouseEvent> {
    let is_dragging = is_dragging.clone();
    let last_mouse_pos = last_mouse_pos.clone();
    let view_box_position = view_box_position.clone();
    let zoom_level = zoom_level.clone();
    let view_box = view_box.clone();

    Callback::from(move |e: MouseEvent| {
        if *is_dragging {
            let (dx, dy) = calculate_mouse_delta(e.client_x(), e.client_y(), &last_mouse_pos);

            if dx.is_nan() || dy.is_nan() {
                log::error!("Invalid mouse movement delta: dx={}, dy={}", dx, dy);
                return;
            }

            last_mouse_pos.set((e.client_x() as f64, e.client_y() as f64));

            let (view_box_width, view_box_height) = calculate_view_box_size(bounds, *zoom_level);
            if view_box_width > 0 && view_box_height > 0 {
                // Scale browser pixels to SVG units so 1px drag = proportional SVG movement
                let scale_x = view_box_width as f64 / display_size.0 as f64;
                let scale_y = view_box_height as f64 / display_size.1 as f64;
                let (new_view_box_x, new_view_box_y) = calculate_new_view_box_position(
                    &view_box_position,
                    dx * scale_x,
                    dy * scale_y,
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
    let view_box_width = ((bounds.1.0 - bounds.0.0) as f64 / zoom_level) as i32;
    let view_box_height = ((bounds.1.1 - bounds.0.1) as f64 / zoom_level) as i32;
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
    let clamped = clamp_view_box_position(
        **view_box_position,
        (dx, dy),
        (view_box_width, view_box_height),
        bounds,
    );
    (clamped.0, clamped.1)
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
    use konnektoren_core::challenges::ChallengeConfig;
    use konnektoren_core::game::Map;
    use yew::callback;
    use yew_preview::prelude::*;

    fn on_select() -> Callback<(Option<ChallengeIndex>, BrowserCoordinate)> {
        callback::Callback::from(
            move |(challenge_index, coordinate): (Option<ChallengeIndex>, BrowserCoordinate)| {
                if let Some(idx) = challenge_index {
                    log::info!("Challenge selected: {}", idx);
                } else {
                    log::info!("Deselected at {} {}", coordinate.0, coordinate.1);
                }
            },
        )
    }

    fn props_background() -> MapComponentProps {
        let mut props = MapComponentProps::default();
        props.current_challenge = 1;
        props.points = Some(100);
        props.on_select_challenge = Some(on_select());
        props.image_src = Some("https://picsum.photos/800".to_string());
        props.width = 800;
        props.height = 800;
        props
    }

    fn props_with_map_bounds() -> MapComponentProps {
        let game_path = konnektoren_core::game::GamePath {
            id: "preview-map-bounds".to_string(),
            name: "Map Bounds Preview".to_string(),
            challenges: vec![
                ChallengeConfig {
                    id: "c1".to_string(),
                    name: "Start".to_string(),
                    position: Some((1, 2)),
                    ..Default::default()
                },
                ChallengeConfig {
                    id: "c2".to_string(),
                    name: "Middle".to_string(),
                    position: Some((5, 4)),
                    ..Default::default()
                },
                ChallengeConfig {
                    id: "c3".to_string(),
                    name: "End".to_string(),
                    position: Some((8, 7)),
                    ..Default::default()
                },
            ],
            map: Some(Map {
                background: String::new(),
                width: 10,
                height: 10,
            }),
        };
        MapComponentProps {
            game_path,
            current_challenge: 0,
            on_select_challenge: Some(on_select()),
            points: Some(50),
            image_src: Some("https://picsum.photos/800".to_string()),
            width: 800,
            height: 800,
        }
    }

    yew_preview::create_preview!(
        MapComponent,
        MapComponentProps::default(),
        ("Background", props_background()),
        ("Map Bounds", props_with_map_bounds())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bounds(x0: i32, y0: i32, x1: i32, y1: i32) -> (SvgCoordinate, SvgCoordinate) {
        (SvgCoordinate(x0, y0), SvgCoordinate(x1, y1))
    }

    // ── clamp_view_box_position ─────────────────────────────────────────────

    #[test]
    fn no_delta_stays_in_place() {
        let b = bounds(0, 0, 100, 100);
        let pos = SvgCoordinate(10, 20);
        let result = clamp_view_box_position(pos, (0.0, 0.0), (50, 50), b);
        assert_eq!(result, SvgCoordinate(10, 20));
    }

    #[test]
    fn delta_moves_view_by_delta() {
        let b = bounds(0, 0, 200, 200);
        let pos = SvgCoordinate(50, 50);
        // delta (10, 5) → pos.0 - 10 = 40, pos.1 - 5 = 45
        let result = clamp_view_box_position(pos, (10.0, 5.0), (50, 50), b);
        assert_eq!(result, SvgCoordinate(40, 45));
    }

    // ── Portrait map (4×5) on landscape screen (vb 5×4 ratio) ───────────────
    // Map SVG: (0,0)-(40,50).  ViewBox sized for landscape display: 50 wide × 32 tall.

    #[test]
    fn portrait_map_landscape_vb_bottom_reachable() {
        let b = bounds(0, 0, 40, 50);
        let vb = (50, 32);
        // max_y = 50 - 32 + SCALE = 28  →  viewBox bottom = 28 + 32 = 60 ≥ 50
        let max_y = b.1.1 - vb.1 + SCALE;
        assert!(
            max_y + vb.1 >= b.1.1,
            "bottom of map not reachable: max_y={max_y}, vb_h={}, map_bottom={}",
            vb.1,
            b.1.1
        );
        // Clamp must produce a valid position when dragging to the bottom
        let at_top = clamp_view_box_position(SvgCoordinate(0, 0), (0.0, -100.0), vb, b);
        assert!(at_top.1 + vb.1 >= b.1.1, "bottom challenge unreachable after full drag");
    }

    // ── Landscape map (5×4) on portrait screen (vb 4×5 ratio) ──────────────
    // Map SVG: (0,0)-(50,40).  ViewBox: 32 wide × 50 tall.

    #[test]
    fn landscape_map_portrait_vb_right_reachable() {
        let b = bounds(0, 0, 50, 40);
        let vb = (32, 50);
        // max_x = 50 - 32 + SCALE = 28  →  viewBox right = 28 + 32 = 60 ≥ 50
        let max_x = b.1.0 - vb.0 + SCALE;
        assert!(
            max_x + vb.0 >= b.1.0,
            "right edge of map not reachable: max_x={max_x}, vb_w={}, map_right={}",
            vb.0,
            b.1.0
        );
        // Drag leftward (negative x delta) to pan right in SVG space
        let at_right = clamp_view_box_position(SvgCoordinate(0, 0), (-200.0, 0.0), vb, b);
        assert!(
            at_right.0 + vb.0 >= b.1.0,
            "right edge unreachable after full drag: pos={}, vb_w={}, map_right={}",
            at_right.0,
            vb.0,
            b.1.0
        );
    }

    // ── ViewBox larger than map: full content still reachable ───────────────

    #[test]
    fn oversized_vb_full_map_stays_visible() {
        // vb_height (62) > map_height (50):
        //   max_y = 50 - 62 + SCALE = -2,  min_y = -10  →  range [-10, -2]
        // The entire map (y 0..50) fits inside any viewBox position in that range.
        let b = bounds(0, 0, 40, 50);
        let vb = (62, 62);
        let result = clamp_view_box_position(SvgCoordinate(0, 0), (0.0, 5.0), vb, b);
        // result.1 must be within [-10, -2] and the viewBox must cover the full map height
        assert!(result.1 >= b.0.1 - SCALE, "position below min");
        assert!(result.1 <= b.1.1 - vb.1 + SCALE, "position above max");
        assert!(result.1 + vb.1 >= b.1.1, "map bottom not covered");
    }

    // ── calculate_view_box_size ─────────────────────────────────────────────

    #[test]
    fn view_box_size_at_zoom_1() {
        let b = bounds(0, 0, 60, 80);
        let (w, h) = calculate_view_box_size(b, 1.0);
        assert_eq!(w, 60);
        assert_eq!(h, 80);
    }

    #[test]
    fn view_box_size_halves_at_zoom_2() {
        let b = bounds(0, 0, 60, 80);
        let (w, h) = calculate_view_box_size(b, 2.0);
        assert_eq!(w, 30);
        assert_eq!(h, 40);
    }

    // ── Non-zero origin bounds ───────────────────────────────────────────────

    #[test]
    fn non_zero_origin_bottom_reachable() {
        // Challenges at SVG (20,10)-(80,90), vb sized for a wide screen
        let b = bounds(20, 10, 80, 90);
        let vb = (80, 50); // wider than span (60), shorter than span (80)
        let max_y = b.1.1 - vb.1 + SCALE;
        let min_y = b.0.1 - SCALE;
        // if range is valid, bottom must be reachable
        if max_y >= min_y {
            assert!(max_y + vb.1 >= b.1.1);
        }
        let at_top = clamp_view_box_position(SvgCoordinate(20, 10), (0.0, -200.0), vb, b);
        assert!(at_top.1 + vb.1 >= b.1.1, "bottom unreachable from non-zero origin");
    }
}
