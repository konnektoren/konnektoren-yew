use super::NavItem;
use crate::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct MenuProps<Route: yew_router::Routable + 'static> {
    pub items: Vec<NavItem<Route>>,
}

#[function_component(NavigationMenu)]
pub fn navigation_menu<Route: yew_router::Routable + std::fmt::Debug + 'static>(
    props: &MenuProps<Route>,
) -> Html {
    let i18n = use_i18n();
    let active_group = use_state(|| None::<usize>);

    #[cfg(feature = "csr")]
    let dropdown_ref = use_node_ref();

    #[cfg(not(feature = "csr"))]
    let dropdown_ref = NodeRef::default(); // Placeholder for SSR

    // Toggle group function
    let toggle_group = {
        let active_group = active_group.clone();
        move |index: usize| {
            let active_group_clone = active_group.clone();
            Callback::from(move |_| {
                if Some(index) == *active_group_clone {
                    active_group_clone.set(None);
                } else {
                    active_group_clone.set(Some(index));
                }
            })
        }
    };

    #[cfg(feature = "csr")]
    let handle_outside_click = {
        use wasm_bindgen::JsCast;
        let active_group = active_group.clone();
        let dropdown_ref = dropdown_ref.clone();

        Callback::from(move |e: MouseEvent| {
            if let Some(dropdown_element) = dropdown_ref.cast::<web_sys::HtmlElement>() {
                let target = e.target();
                if let Some(target_element) =
                    target.and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                {
                    if !dropdown_element.contains(Some(&target_element)) {
                        active_group.set(None);
                    }
                }
            }
        })
    };

    // Add event listener to document when active_group changes
    #[cfg(feature = "csr")]
    {
        use wasm_bindgen::JsCast;
        use yew::use_effect_with;

        use_effect_with(active_group.clone(), move |active_group| {
            let cleanup_fn = if active_group.is_some() {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let handle_outside_click_clone = handle_outside_click.clone();

                let listener = gloo::events::EventListener::new(&document, "mousedown", move |e| {
                    let event = e.clone().dyn_into::<web_sys::MouseEvent>().unwrap();
                    handle_outside_click_clone.emit(event);
                });

                // Return a function that drops the listener
                Box::new(move || drop(listener)) as Box<dyn FnOnce()>
            } else {
                // No cleanup needed if no active group
                Box::new(|| {}) as Box<dyn FnOnce()>
            };

            cleanup_fn
        });
    }

    // Get only group items for dropdown logic
    let group_items: Vec<_> = props
        .items
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            if let NavItem::Group { .. } = item {
                Some((idx, item))
            } else {
                None
            }
        })
        .collect();

    html! {
        <div class="navigation-wrapper">
            {
                if let Some(active_idx) = *active_group {
                    if let Some(&(_, NavItem::Group { items, .. })) = group_items.get(active_idx) {
                        html! {
                            <div class="navigation-dropdown" ref={dropdown_ref}>
                                <nav>
                                    {
                                        items.iter().map(|item| {
                                            match item {
                                                NavItem::Route { name, route, icon } => html! {
                                                    <Link<Route> to={route.clone()}>
                                                        <i class={*icon}></i>
                                                        <span>{ i18n.t(name) }</span>
                                                    </Link<Route>>
                                                },
                                                NavItem::Component { component } => html! {
                                                    <div class="nav-item-component">{ component.clone() }</div>
                                                },
                                                NavItem::Group { .. } => html! {
                                                    // Nested groups not rendered in dropdown
                                                    <div class="nav-item-error">{ "Nested groups not supported" }</div>
                                                }
                                            }
                                        }).collect::<Html>()
                                    }
                                </nav>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                } else {
                    html! {}
                }
            }
            <nav class="navigation">
                {
                    props.items.iter().enumerate().map(|(idx, item)| {
                        match item {
                            NavItem::Route { name, route, icon } => html! {
                                <Link<Route> to={route.clone()} >
                                    <i class={*icon}></i>
                                    <span>{ i18n.t(name) }</span>
                                </Link<Route>>
                            },
                            NavItem::Component { component } => html! {
                                <div class="nav-extra-inline">{ component.clone() }</div>
                            },
                            NavItem::Group { name, icon, .. } => {
                                // Find group index in the filtered group_items
                                let group_idx = group_items.iter().position(|&(original_idx, _)| original_idx == idx)
                                    .unwrap_or(0);
                                let is_active = Some(group_idx) == *active_group;
                                html! {
                                    <button
                                        class={if is_active { "nav-group active" } else { "nav-group" }}
                                        onclick={toggle_group(group_idx)}
                                    >
                                        <i class={*icon}></i>
                                        <span>{ i18n.t(name) }</span>
                                    </button>
                                }
                            }
                        }
                    }).collect::<Html>()
                }
            </nav>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
pub mod preview {
    use super::*;
    use yew::prelude::*;
    use yew_preview::prelude::*;

    #[derive(Routable, PartialEq, Clone, Debug, Default)]
    pub enum Route {
        #[default]
        #[at("/")]
        Root,
        #[at("/home/")]
        Home,
    }

    fn get_nav_items() -> Vec<NavItem<Route>> {
        vec![
            NavItem::Group {
                name: "Home",
                icon: "fas fa-home",
                items: vec![
                    NavItem::Route {
                        name: "Root",
                        route: Route::Root,
                        icon: "fas fa-home",
                    },
                    NavItem::Route {
                        name: "Home",
                        route: Route::Home,
                        icon: "fas fa-home",
                    },
                ],
            },
            NavItem::Component {
                component: html! { <span class="inbox-icon">{"📬"}</span> },
            },
            NavItem::Group {
                name: "Mail",
                icon: "fas fa-briefcase",
                items: vec![
                    NavItem::Route {
                        name: "Root",
                        route: Route::Root,
                        icon: "fas fa-home",
                    },
                    NavItem::Component {
                        component: html! { <span>{"Custom Component"}</span> },
                    },
                ],
            },
            NavItem::Route {
                name: "Direct Link",
                route: Route::Home,
                icon: "fas fa-link",
            },
        ]
    }

    #[derive(Default, PartialEq, Properties)]
    pub struct ExampleMenuProps;

    #[function_component(ExampleMenu)]
    pub fn example_menu(_props: &ExampleMenuProps) -> Html {
        html! {
            <NavigationMenu<Route> items={get_nav_items()} />
        }
    }

    yew_preview::create_preview!(ExampleMenu, ExampleMenuProps::default(),);
}
