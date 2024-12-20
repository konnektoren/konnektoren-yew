use super::{NavExtra, NavGroup};
use crate::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct MenuProps<Route: yew_router::Routable + 'static> {
    pub groups: Vec<NavGroup<Route>>,
}

#[function_component(NavigationMenu)]
pub fn navigation_menu<Route: yew_router::Routable + 'static>(props: &MenuProps<Route>) -> Html {
    let i18n = use_i18n();
    let active_group = use_state(|| None::<usize>);

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

    html! {
        <>
            if let Some(active_idx) = *active_group {
                <div class="navigation-dropdown" onclick={Callback::from({
                    let active_group = active_group.clone();
                    move |_| active_group.set(None)
                })}>
                    <nav>
                        {
                            props.groups[active_idx].items.iter().map(|item| {
                                html! {
                                    <Link<Route> to={item.route.clone()} >
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
        </>
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
