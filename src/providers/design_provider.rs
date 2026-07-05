use yew::prelude::*;

use crate::model::{Design, DesignMode};

/// localStorage key for the persisted user design preference.
pub const DESIGN_MODE_STORAGE_KEY: &str = "konnektoren.design.mode";

#[derive(Clone, PartialEq)]
pub struct DesignContext {
    /// Effective design — read this in components to branch per design.
    pub design: UseStateHandle<Design>,
    /// Configuration — set this (e.g. from settings) to override detection.
    pub mode: UseStateHandle<DesignMode>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct DesignProviderProps {
    pub children: Children,
    /// Viewport width in px at or below which `Design::Mobile` is detected.
    #[prop_or(768)]
    pub mobile_breakpoint: u32,
    /// App-level override: skips detection and user configuration.
    #[prop_or_default]
    pub forced: Option<Design>,
}

#[cfg(feature = "csr")]
fn media_query(mobile_breakpoint: u32) -> Option<web_sys::MediaQueryList> {
    gloo::utils::window()
        .match_media(&format!("(max-width: {mobile_breakpoint}px)"))
        .ok()
        .flatten()
}

#[cfg(feature = "csr")]
fn screen_matches_mobile(mobile_breakpoint: u32) -> bool {
    use wasm_bindgen::JsValue;

    let window = gloo::utils::window();
    let Ok(screen) = js_sys::Reflect::get(window.as_ref(), &JsValue::from_str("screen")) else {
        return false;
    };
    let Ok(width) = js_sys::Reflect::get(&screen, &JsValue::from_str("width")) else {
        return false;
    };

    width
        .as_f64()
        .is_some_and(|width| width <= f64::from(mobile_breakpoint))
}

/// Screen-size based detection; falls back to the body class (which an
/// inline script or the server may have set before the app started).
fn detect_design(_mobile_breakpoint: u32) -> Design {
    #[cfg(feature = "csr")]
    {
        let detected = match media_query(_mobile_breakpoint) {
            Some(mql) if mql.matches() || screen_matches_mobile(_mobile_breakpoint) => {
                Design::Mobile
            }
            Some(_) => Design::Desktop,
            None if screen_matches_mobile(_mobile_breakpoint) => Design::Mobile,
            None => Design::get_from_body(),
        };
        tracing::debug!(
            "Design detected from screen (breakpoint {_mobile_breakpoint}px): {detected:?}"
        );
        detected
    }

    // Mobile-first: most users are on phones, so prerendered HTML should
    // match what hydration will most likely keep.
    #[cfg(not(feature = "csr"))]
    {
        Design::Mobile
    }
}

#[cfg(feature = "csr")]
fn update_detected_design(detected: &UseStateHandle<Design>, mobile_breakpoint: u32) {
    let new_design = detect_design(mobile_breakpoint);
    if **detected != new_design {
        tracing::info!(
            "Detected design change: {:?} -> {:?}",
            **detected,
            new_design
        );
        detected.set(new_design);
    }
}

fn load_mode() -> DesignMode {
    if let Some(design) = Design::configured_from_body() {
        let mode = DesignMode::Fixed(design);
        tracing::debug!("Design mode loaded from body: {mode:?}");
        return mode;
    }

    #[cfg(all(feature = "csr", feature = "storage"))]
    {
        use gloo::storage::{LocalStorage, Storage};
        if let Ok(stored) = LocalStorage::get::<String>(DESIGN_MODE_STORAGE_KEY) {
            let mode = DesignMode::from_storage_value(&stored);
            tracing::debug!("Design mode loaded from storage: {mode:?}");
            return mode;
        }
    }
    DesignMode::default()
}

#[allow(unused_variables)]
fn save_mode(mode: &DesignMode) {
    #[cfg(all(feature = "csr", feature = "storage"))]
    {
        use gloo::storage::{LocalStorage, Storage};
        if let Err(err) = LocalStorage::set(DESIGN_MODE_STORAGE_KEY, mode.as_str()) {
            tracing::warn!("Failed to persist design mode: {err:?}");
        }
    }
}

/// Priority: `forced` prop > configured [`DesignMode::Fixed`] > screen detection.
fn effective_design(forced: &Option<Design>, mode: &DesignMode, detected: &Design) -> Design {
    if let Some(design) = forced {
        return design.clone();
    }
    match mode {
        DesignMode::Fixed(design) => design.clone(),
        DesignMode::Auto => detected.clone(),
    }
}

#[function_component(DesignProvider)]
pub fn design_provider(props: &DesignProviderProps) -> Html {
    // NOTE: all hook calls stay outside cfg blocks (hook ordering).
    let mode = use_state(load_mode);
    let mode_loaded = use_mut_ref(|| true);
    let detected = {
        let mobile_breakpoint = props.mobile_breakpoint;
        use_state(move || detect_design(mobile_breakpoint))
    };
    let design = {
        let forced = props.forced.clone();
        let mode = (*mode).clone();
        let detected = (*detected).clone();
        use_state(move || effective_design(&forced, &mode, &detected))
    };

    // Follow breakpoint crossings, browser resizes, and mobile orientation
    // changes. Some mobile pages without viewport metadata keep a wide layout
    // viewport, so resize/orientation events re-run full detection.
    #[cfg(feature = "csr")]
    {
        let detected = detected.clone();
        use_effect_with(props.mobile_breakpoint, move |mobile_breakpoint| {
            let mobile_breakpoint = *mobile_breakpoint;
            update_detected_design(&detected, mobile_breakpoint);

            let media_listener = media_query(mobile_breakpoint).map(|mql| {
                let detected = detected.clone();
                gloo::events::EventListener::new(&mql, "change", move |event| {
                    use wasm_bindgen::JsCast;
                    if let Some(event) = event.dyn_ref::<web_sys::MediaQueryListEvent>() {
                        tracing::debug!("Design media query changed: {}", event.matches());
                    }
                    update_detected_design(&detected, mobile_breakpoint);
                })
            });

            let resize_listener = {
                let detected = detected.clone();
                gloo::events::EventListener::new(&gloo::utils::window(), "resize", move |_| {
                    update_detected_design(&detected, mobile_breakpoint);
                })
            };

            let orientation_listener = {
                let detected = detected.clone();
                gloo::events::EventListener::new(
                    &gloo::utils::window(),
                    "orientationchange",
                    move |_| {
                        update_detected_design(&detected, mobile_breakpoint);
                    },
                )
            };

            move || {
                drop(media_listener);
                drop(resize_listener);
                drop(orientation_listener);
            }
        });
    }

    // Resolve the effective design whenever an input changes.
    {
        let design = design.clone();
        use_effect_with(
            (props.forced.clone(), (*mode).clone(), (*detected).clone()),
            move |(forced, mode, detected)| {
                let effective = effective_design(forced, mode, detected);
                if *design != effective {
                    tracing::info!(
                        "Design change: {:?} -> {:?} (mode: {:?}, detected: {:?})",
                        *design,
                        effective,
                        mode,
                        detected
                    );
                    design.set(effective);
                }
            },
        );
    }

    // Keep the body class in sync so per-design CSS keeps working.
    #[cfg(feature = "csr")]
    {
        use_effect_with((*design).clone(), move |design| {
            design.apply_to_body();
        });
    }

    // Persist the configuration — only on actual changes, not for the
    // mode that was just loaded at startup.
    {
        use_effect_with((*mode).clone(), move |mode| {
            if *mode_loaded.borrow() {
                *mode_loaded.borrow_mut() = false;
                tracing::debug!("Design mode active: {mode:?}");
            } else {
                tracing::info!("Design mode configured: {mode:?}");
                save_mode(mode);
            }
        });
    }

    let context = DesignContext {
        design: design.clone(),
        mode: mode.clone(),
    };

    html! {
        <ContextProvider<DesignContext> context={context}>
            {props.children.clone()}
        </ContextProvider<DesignContext>>
    }
}

/// Effective design for rendering. To change the design, use
/// [`use_design_mode`] instead — direct `set()` calls on this handle are
/// overwritten by the provider.
#[hook]
pub fn use_design() -> UseStateHandle<Design> {
    use_context::<DesignContext>()
        .expect("use_design must be used within a DesignProvider")
        .design
}

/// Design configuration: `Auto` (follow screen size) or `Fixed(Design)`.
/// Persisted to localStorage when the `storage` feature is enabled.
#[hook]
pub fn use_design_mode() -> UseStateHandle<DesignMode> {
    use_context::<DesignContext>()
        .expect("use_design_mode must be used within a DesignProvider")
        .mode
}
