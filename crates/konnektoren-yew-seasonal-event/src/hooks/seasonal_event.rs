use crate::model::SeasonalEvent;
use yew::prelude::*;

#[cfg(all(feature = "csr", target_arch = "wasm32"))]
fn is_visible<E: SeasonalEvent>() -> bool {
    let now = js_sys::Date::new_0();
    E::visible_time().contains(now.get_time() as i64, now.get_month() + 1, now.get_date())
}

#[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
fn server_is_visible<E: SeasonalEvent>() -> bool {
    use chrono::Datelike;
    let now = chrono::Local::now();
    E::visible_time().contains(now.timestamp_millis(), now.month(), now.day())
}

/// Updates while mounted. Forced visibility also works during server rendering.
#[hook]
pub fn use_seasonal_event<E: SeasonalEvent + 'static>(force_visible: bool) -> bool {
    #[cfg(all(feature = "csr", target_arch = "wasm32"))]
    let active = use_state_eq(is_visible::<E>);
    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    let active = use_state_eq(server_is_visible::<E>);
    #[cfg(all(
        not(all(feature = "csr", target_arch = "wasm32")),
        not(all(feature = "ssr", not(target_arch = "wasm32")))
    ))]
    let active = use_state_eq(|| false);

    #[cfg(all(feature = "csr", target_arch = "wasm32"))]
    {
        let update = active.clone();
        use_effect_with((), move |_| {
            update.set(is_visible::<E>());
            let timer = gloo::timers::callback::Interval::new(1_000, move || {
                update.set(is_visible::<E>());
            });
            move || drop(timer)
        });
    }

    force_visible || *active
}
