use super::*;
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
            component: html! { <crate::components::SelectTheme /> },
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
    let tools = html! { <crate::components::SelectLanguage /> };
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
        "theme selector slot",
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
        "header with language selector",
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
