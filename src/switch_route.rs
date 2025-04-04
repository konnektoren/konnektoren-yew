use crate::pages::{About, Example, Home};
use crate::route::Route;
use yew::prelude::*;

pub fn switch_route(route: Route) -> Html {
    match route {
        Route::Root => html! {<Home />},
        Route::Home => html! {<Home />},
        Route::About => html! {<About />},
        Route::Example => html! {<Example />},
    }
}
