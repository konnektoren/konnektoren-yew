use super::{NavExtra, NavGroup};
use crate::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct MenuProps<Route: yew_router::Routable + 'static> {
    pub groups: Vec<NavGroup<Route>>,
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

    html! {
        <div class="navigation-wrapper">
            {
                if let Some(active_idx) = *active_group {
                    html! {
                        <div class="navigation-dropdown" ref={dropdown_ref}>
                            <nav>
                                {
                                    props.groups[active_idx].items.iter().map(|item| {
                                        html! {
                                            <Link<Route> to={item.route.clone()}>
                                                <i class={item.icon}></i>
                                                <span>{ i18n.t(item.name) }</span>
                                            </Link<Route>>
                                        }
                                    }).collect::<Html>()
                                }
                                {
                                    if let Some(extras) = &props.groups[active_idx].extras {
                                        html! {
                                            <div class="nav-extras">
                                                {
                                                    extras.iter().map(|extra| {
                                                        match extra {
                                                            NavExtra::Component(component) => component.clone(),
                                                        }
                                                    }).collect::<Html>()
                                                }
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </nav>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            <nav class="navigation">
                {
                    props.groups.iter().enumerate().map(|(idx, group)| {
                        let is_active = Some(idx) == *active_group;
                        html! {
                            <button
                                class={if is_active { "nav-group active" } else { "nav-group" }}
                                onclick={toggle_group(idx)}
                            >
                                <i class={group.icon}></i>
                                <span>{ i18n.t(group.name) }</span>
                            </button>
                        }
                    }).collect::<Html>()
                }
            </nav>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
pub mod preview {
    use super::super::NavItem;
    use super::*;
    use yew_preview::prelude::*;

    #[derive(Routable, PartialEq, Clone, Debug, Default)]
    pub enum Route {
        #[default]
        #[at("/")]
        Root,
        #[at("/home/")]
        Home,
    }

    fn get_groups() -> Vec<NavGroup<Route>> {
        vec![
            NavGroup {
                name: "Home",
                icon: "fas fa-home",
                items: vec![
                    NavItem {
                        name: "Root",
                        route: Route::Root,
                        icon: "fas fa-home",
                    },
                    NavItem {
                        name: "Home",
                        route: Route::Home,
                        icon: "fas fa-home",
                    },
                ],
                extras: None,
            },
            NavGroup {
                name: "Mail",
                icon: "fas fa-briefcase",
                items: vec![
                    NavItem {
                        name: "Root",
                        route: Route::Root,
                        icon: "fas fa-home",
                    },
                    NavItem {
                        name: "Home",
                        route: Route::Home,
                        icon: "fas fa-home",
                    },
                ],
                extras: None,
            },
        ]
    }

    #[derive(Default, PartialEq, Properties)]
    pub struct ExampleMenuProps;

    #[function_component(ExampleMenu)]
    pub fn exampe_menu(_props: &ExampleMenuProps) -> Html {
        html! {
            <NavigationMenu<Route> groups={get_groups()} />
        }
    }

    yew_preview::create_preview!(ExampleMenu, ExampleMenuProps::default(),);
}
