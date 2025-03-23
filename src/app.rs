use crate::providers::ClientSideRouter;
use crate::route::Route;
use crate::switch_route::switch_route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ClientSideRouter>
            <Switch<Route> render={switch_route} />
        </ClientSideRouter>
    }
}
