use crate::route::Route;
use crate::switch_route::switch_route;
use yew::prelude::*;

/// Direct component renderer that bypasses router
#[derive(Properties, PartialEq)]
pub struct DirectRenderProps {
    pub route: Route,
}

#[function_component(DirectRender)]
pub fn direct_render(props: &DirectRenderProps) -> Html {
    switch_route(props.route.clone())
}
