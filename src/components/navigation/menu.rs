use super::{NavItem, NavigationLinks};
use crate::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct MenuProps<Route: Routable + 'static> {
    pub items: Vec<NavItem<Route>>,
    #[prop_or_else(|| "Main navigation".into())]
    pub label: AttrValue,
}

/// Bottom navigation with grouped menus and arbitrary component slots.
#[function_component(NavigationMenu)]
pub fn navigation_menu<Route: Routable + 'static>(props: &MenuProps<Route>) -> Html {
    let i18n = use_i18n();
    let active_group = use_state(|| None::<usize>);
    let open = use_state(|| false);
    let root = use_node_ref();
    let current = use_route::<Route>();
    super::use_navigation_dismiss(root.clone(), open.clone());

    {
        let open = open.clone();
        use_effect_with(current.clone(), move |_| open.set(false));
    }
    let close = {
        let open = open.clone();
        Callback::from(move |()| open.set(false))
    };
    let selected = if *open {
        active_group.and_then(|index| props.items.get(index))
    } else {
        None
    };

    html! {
        <div class="navigation-menu" ref={root}>
            if let Some(NavItem::Group { name, items, .. }) = selected {
                <nav class="navigation-menu__dropdown" aria-label={i18n.t(name)}>
                    <NavigationLinks<Route> items={items.clone()} on_navigate={close.clone()} />
                </nav>
            }
            <nav class="navigation-menu__bar" aria-label={i18n.t(&props.label)}>
                {for props.items.iter().enumerate().map(|(index, item)| match item {
                    NavItem::Group { name, icon, .. } => {
                        let expanded = *open && *active_group == Some(index);
                        let onclick = {
                            let open = open.clone();
                            let active_group = active_group.clone();
                            Callback::from(move |_: MouseEvent| {
                                active_group.set(Some(index));
                                open.set(!expanded);
                            })
                        };
                        html! {
                            <button type="button" class={classes!("navigation-menu__group",
                                expanded.then_some("navigation-menu__group--open"))}
                                aria-expanded={expanded.to_string()} {onclick}>
                                <span class="navigation-menu__icon" aria-hidden="true">{icon.clone()}</span>
                                <span>{i18n.t(name)}</span>
                            </button>
                        }
                    },
                    _ => html! { <NavigationLinks<Route> items={vec![item.clone()]} on_navigate={close.clone()} /> },
                })}
            </nav>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
pub mod preview {
    // Retain the original preview path for downstream preview catalogs.
    pub use super::super::preview::ExampleMenu;
}
