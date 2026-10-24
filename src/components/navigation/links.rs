use super::{NavItem, NavigationLink};
use crate::i18n::use_i18n;
use yew::prelude::*;
use yew_router::Routable;

#[derive(Properties, PartialEq)]
pub struct NavigationLinksProps<Route: Routable + 'static> {
    pub items: Vec<NavItem<Route>>,
    #[prop_or_default]
    pub active_route: Option<Route>,
    #[prop_or_default]
    pub compact: bool,
    #[prop_or_default]
    pub on_navigate: Callback<()>,
}

/// Route, component, and nested group rendering shared by every navigation layout.
#[function_component(NavigationLinks)]
pub fn navigation_links<Route: Routable + 'static>(props: &NavigationLinksProps<Route>) -> Html {
    let i18n = use_i18n();
    html! {
        <>
            {for props.items.iter().map(|item| match item {
                NavItem::Route { name, route, icon } => html! {
                    <NavigationLink<Route> to={route.clone()} name={*name} icon={icon.clone()}
                        active_route={props.active_route.clone()} compact={props.compact}
                        on_navigate={props.on_navigate.clone()} />
                },
                NavItem::Component { component } => html! {
                    <div class="navigation-component">{component.clone()}</div>
                },
                NavItem::Group { name, icon, items } => html! {
                    <details class="navigation-group">
                        <summary class="navigation-group__trigger">
                            <span class="navigation-link__icon" aria-hidden="true">{icon.clone()}</span>
                            <span>{i18n.t(name)}</span>
                            <span class="navigation-group__arrow" aria-hidden="true">{"▸"}</span>
                        </summary>
                        <div class="navigation-group__submenu">
                            <NavigationLinks<Route> items={items.clone()} active_route={props.active_route.clone()}
                                on_navigate={props.on_navigate.clone()} />
                        </div>
                    </details>
                },
            })}
        </>
    }
}
