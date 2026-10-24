use super::{NavItem, NavigationLinks};
use crate::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::{Routable, use_route};

#[derive(Clone, Copy, Default, PartialEq)]
pub enum NavigationBarLayout {
    #[default]
    Inline,
    Bottom,
    Toolbar,
}

#[derive(Properties, PartialEq)]
pub struct NavigationBarProps<Route: Routable + 'static> {
    pub items: Vec<NavItem<Route>>,
    #[prop_or_default]
    pub layout: NavigationBarLayout,
    #[prop_or_default]
    pub brand: Html,
    #[prop_or_default]
    pub actions: Html,
    #[prop_or_default]
    pub secondary_items: Vec<NavItem<Route>>,
    #[prop_or_else(|| "Main navigation".into())]
    pub label: AttrValue,
    #[prop_or_else(|| "More".into())]
    pub secondary_label: AttrValue,
    #[prop_or_default]
    pub active_route: Option<Route>,
}

/// Inline links, a bottom bar, or a toolbar with brand, overflow, and action slots.
#[function_component(NavigationBar)]
pub fn navigation_bar<Route: Routable + 'static>(props: &NavigationBarProps<Route>) -> Html {
    let i18n = use_i18n();
    let menu_open = use_state(|| false);
    let root = use_node_ref();
    let current = use_route::<Route>();
    {
        let menu_open = menu_open.clone();
        use_effect_with(current, move |_| menu_open.set(false));
    }
    super::use_navigation_dismiss(root.clone(), menu_open.clone());
    let on_navigate = {
        let menu_open = menu_open.clone();
        Callback::from(move |()| menu_open.set(false))
    };
    let toggle = {
        let menu_open = menu_open.clone();
        Callback::from(move |_: MouseEvent| menu_open.set(!*menu_open))
    };
    let toolbar = props.layout == NavigationBarLayout::Toolbar;
    let layout_class = match props.layout {
        NavigationBarLayout::Inline => "navigation-bar--inline",
        NavigationBarLayout::Bottom => "navigation-bar--bottom",
        NavigationBarLayout::Toolbar => "navigation-bar--toolbar",
    };
    html! {
        <div ref={root} class={classes!("navigation-bar", layout_class)}>
            if toolbar { <div class="navigation-bar__brand">{props.brand.clone()}</div> }
            <nav class="navigation-bar__primary" aria-label={i18n.t(&props.label)}>
                <NavigationLinks<Route> items={props.items.clone()} compact={toolbar}
                    active_route={props.active_route.clone()} on_navigate={on_navigate.clone()} />
            </nav>
            if !props.secondary_items.is_empty() {
                <div class="navigation-bar__overflow">
                    <button type="button" class="navigation-bar__trigger" aria-expanded={menu_open.to_string()}
                        aria-label={i18n.t(&props.secondary_label)} onclick={toggle}>
                        <span aria-hidden="true">{"⋯"}</span>
                    </button>
                    if *menu_open {
                        <nav class="navigation-bar__secondary" aria-label={i18n.t(&props.secondary_label)}>
                            <NavigationLinks<Route> items={props.secondary_items.clone()} compact={toolbar}
                                active_route={props.active_route.clone()} on_navigate={on_navigate} />
                        </nav>
                    }
                </div>
            }
            if toolbar { <div class="navigation-bar__actions">{props.actions.clone()}</div> }
        </div>
    }
}
