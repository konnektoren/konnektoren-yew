use crate::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavigationLinkProps<Route: Routable + 'static> {
    pub to: Route,
    /// Translation key for the visible and accessible label.
    pub name: AttrValue,
    #[prop_or_default]
    pub icon: Html,
    /// Override exact-route matching, for example for a section's child routes.
    #[prop_or_default]
    pub active_route: Option<Route>,
    #[prop_or_default]
    pub compact: bool,
    #[prop_or_default]
    pub on_navigate: Callback<()>,
}

/// A route link with shared icon, label, active state, and accessibility styling.
#[function_component(NavigationLink)]
pub fn navigation_link<Route: Routable + 'static>(props: &NavigationLinkProps<Route>) -> Html {
    let i18n = use_i18n();
    let current = use_route::<Route>();
    let active = props.active_route.as_ref().or(current.as_ref()) == Some(&props.to);
    let name = i18n.t(&props.name);

    #[cfg(feature = "csr")]
    let navigator = yew_router::hooks::use_navigator();

    #[cfg(feature = "csr")]
    let href = navigator.as_ref().map_or_else(
        || props.to.to_path(),
        |navigator| {
            let path = props.to.to_path();
            let mut href = match navigator.basename() {
                Some(base) => format!("{base}{path}"),
                None => path,
            };
            if navigator.kind() == yew_router::navigator::NavigatorKind::Hash {
                href.insert(0, '#');
            }
            href
        },
    );
    #[cfg(not(feature = "csr"))]
    let href = yew_router::utils::combine_with_base_url(&props.to.to_path());

    let onclick = {
        let on_navigate = props.on_navigate.clone();
        #[cfg(feature = "csr")]
        let to = props.to.clone();
        Callback::from(move |event: MouseEvent| {
            if event.default_prevented()
                || event.button() != 0
                || event.ctrl_key()
                || event.meta_key()
                || event.shift_key()
                || event.alt_key()
            {
                return;
            }
            #[cfg(feature = "csr")]
            if let Some(navigator) = &navigator {
                event.prevent_default();
                navigator.push(&to);
            }
            on_navigate.emit(());
        })
    };

    html! {
        <a {href} {onclick}
            class={classes!("navigation-link", active.then_some("navigation-link--active"), props.compact.then_some("navigation-link--compact"))}
            aria-current={active.then_some("page")}
            title={props.compact.then_some(name.clone())}>
            <span class="navigation-link__icon" aria-hidden="true">{props.icon.clone()}</span>
            <span class="navigation-link__label">{name}</span>
        </a>
    }
}
