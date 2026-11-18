use super::*;
use konnektoren_yew_i18n::use_i18n;
use yew::prelude::*;
use yew_preview::prelude::*;
use yew_router::Routable;

#[derive(Routable, PartialEq, Clone, Debug, Default)]
pub enum Route {
    #[default]
    #[at("/")]
    Home,
    #[at("/settings/")]
    Settings,
    #[at("/explore/")]
    Explore,
    #[at("/library/")]
    Library,
}

fn links() -> Vec<NavItem<Route>> {
    vec![
        NavItem::Route {
            name: "Home",
            route: Route::Home,
            icon: html! { <svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="m3 10 9-7 9 7v11h-7v-7h-4v7H3Z" /></svg> },
        },
        NavItem::Route {
            name: "Settings",
            route: Route::Settings,
            icon: html! { <i class="fa-solid fa-cog" /> },
        },
    ]
}

#[derive(Properties, PartialEq, Default)]
pub struct ExampleMenuProps {
    pub nested: bool,
    pub with_component: bool,
}

#[function_component(ExampleMenu)]
pub fn example_menu(props: &ExampleMenuProps) -> Html {
    let mut items = links();
    let children = if props.nested {
        vec![NavItem::Group {
            name: "Settings",
            icon: html! { <span>{"⚙"}</span> },
            items: links(),
        }]
    } else {
        links()
    };
    items.push(NavItem::Group {
        name: "More",
        icon: html! { <span>{"⋯"}</span> },
        items: children,
    });
    if props.with_component {
        items.push(NavItem::Component {
            component: html! { <button type="button">{"Theme"}</button> },
        });
    }
    html! { <div class="navigation-preview"><NavigationMenu<Route> {items} /></div> }
}

#[derive(Properties, PartialEq, Default)]
pub struct ExampleNavigationProps {
    pub layout: NavigationBarLayout,
    pub header: bool,
    pub nested: bool,
}

#[function_component(ExampleNavigation)]
pub fn example_navigation(props: &ExampleNavigationProps) -> Html {
    let brand = html! { <a href="/">{"Konnektoren"}</a> };
    let tools = html! { <button type="button">{"Language"}</button> };
    let secondary_items = if props.nested {
        vec![
            NavItem::Route {
                name: "Home",
                route: Route::Home,
                icon: html! { <span>{"⌂"}</span> },
            },
            NavItem::Group {
                name: "Settings",
                icon: html! { <i class="fa-solid fa-cog" /> },
                items: links(),
            },
        ]
    } else {
        links()
    };
    let navigation = if props.header {
        html! {
            <NavigationHeader {brand} {tools} content_target="preview-main">
                <NavigationBar<Route> items={links()} />
            </NavigationHeader>
        }
    } else {
        html! {
            <NavigationBar<Route> items={links()} layout={props.layout} {brand}
                {secondary_items} actions={html! { <NavigationLinks<Route> items={links()} compact={true} /> }} />
        }
    };
    html! { <div class="navigation-preview">{navigation}<div id="preview-main" tabindex="-1" /></div> }
}

#[derive(Properties, PartialEq, Default)]
pub struct ExampleFavouritesProps {}

/// Demonstrates keeping favourite routes in the primary navigation.
#[function_component(ExampleFavourites)]
pub fn example_favourites(_props: &ExampleFavouritesProps) -> Html {
    let i18n = use_i18n();
    let selected = use_state(|| vec![Route::Explore]);
    let options = [
        ("Explore", Route::Explore, "★"),
        ("Library", Route::Library, "♫"),
        ("Settings", Route::Settings, "⚙"),
    ];
    let item = |name, route, icon| NavItem::Route {
        name,
        route,
        icon: html! { <span>{icon}</span> },
    };
    let mut primary = vec![item("Home", Route::Home, "⌂")];
    primary.extend(selected.iter().filter_map(|route| {
        options
            .iter()
            .find(|(_, option, _)| option == route)
            .map(|(name, route, icon)| item(*name, route.clone(), *icon))
    }));
    let secondary: Vec<_> = options
        .iter()
        .map(|(name, route, icon)| item(*name, route.clone(), *icon))
        .collect();
    html! {
        <div class="navigation-preview navigation-preview--favourites">
            <NavigationHeader brand={html! { <a href="/">{"Konnektoren"}</a> }}>
                <NavigationBar<Route> items={primary.clone()} secondary_items={secondary.clone()} secondary_label="Others" />
            </NavigationHeader>
            <NavigationBar<Route> layout={NavigationBarLayout::Bottom} items={primary} secondary_items={secondary} secondary_label="Others" />
            <section class="navigation-preview__favourites">
                <p>{i18n.t("Choose your favourites for your navigation.")}</p>
                <div class="navigation-preview__selection" aria-label={i18n.t("Navigation preview")}>
                    <p class="navigation-preview__title">{i18n.t("Navigation preview")}</p>
                    <div class="navigation-preview__queue">
                        <span class="navigation-preview__item">{i18n.t("Home")}</span>
                        {for selected.iter().enumerate().filter_map(|(index, route)| {
                            let (name, _, icon) = options.iter().find(|(_, option, _)| option == route)?;
                            let earlier = selected.clone();
                            let later = selected.clone();
                            let earlier_label = format!("{}: {}", i18n.t("Move earlier"), i18n.t(name));
                            let later_label = format!("{}: {}", i18n.t("Move later"), i18n.t(name));
                            Some(html! {
                                <span class="navigation-preview__item">
                                    <span aria-hidden="true">{*icon}</span>{i18n.t(name)}
                                    <button type="button" class="navigation-preview__move" aria-label={earlier_label} disabled={index == 0}
                                        onclick={Callback::from(move |_| { let mut next = (*earlier).clone(); next.swap(index, index - 1); earlier.set(next); })}>{"←"}</button>
                                    <button type="button" class="navigation-preview__move" aria-label={later_label} disabled={index + 1 == selected.len()}
                                        onclick={Callback::from(move |_| { let mut next = (*later).clone(); next.swap(index, index + 1); later.set(next); })}>{"→"}</button>
                                </span>
                            })
                        })}
                        <span class="navigation-preview__item">{i18n.t("Others")}</span>
                    </div>
                </div>
                <fieldset class="navigation-preview__choices">
                    <legend class="navigation-preview__title">{i18n.t("Favourites")}</legend>
                    {for options.into_iter().map(|(name, route, icon)| {
                        let checked = selected.contains(&route);
                        let selected = selected.clone();
                        let onchange = Callback::from(move |_| {
                            let mut next = (*selected).clone();
                            if let Some(index) = next.iter().position(|item| item == &route) {
                                next.remove(index);
                            } else {
                                next.push(route.clone());
                            }
                            selected.set(next);
                        });
                        html! { <label class="navigation-preview__choice"><input type="checkbox" class="checkbox checkbox-primary" {checked} {onchange} /><span aria-hidden="true">{icon}</span>{i18n.t(name)}</label> }
                    })}
                </fieldset>
            </section>
        </div>
    }
}

yew_preview::create_preview!(ExampleFavourites, ExampleFavouritesProps::default(),);

yew_preview::create_preview!(
    ExampleMenu,
    ExampleMenuProps::default(),
    (
        "nested groups",
        ExampleMenuProps {
            nested: true,
            with_component: false
        }
    ),
    (
        "component slot",
        ExampleMenuProps {
            nested: false,
            with_component: true
        }
    ),
);
yew_preview::create_preview!(
    ExampleNavigation,
    ExampleNavigationProps {
        layout: NavigationBarLayout::Inline,
        header: false,
        nested: false
    },
    (
        "header with tool slot",
        ExampleNavigationProps {
            layout: NavigationBarLayout::Inline,
            header: true,
            nested: false
        }
    ),
    (
        "desktop toolbar",
        ExampleNavigationProps {
            layout: NavigationBarLayout::Toolbar,
            header: false,
            nested: false
        }
    ),
    (
        "bottom bar",
        ExampleNavigationProps {
            layout: NavigationBarLayout::Bottom,
            header: false,
            nested: false
        }
    ),
    (
        "ellipsis with nested submenu",
        ExampleNavigationProps {
            layout: NavigationBarLayout::Toolbar,
            header: false,
            nested: true
        }
    ),
);
