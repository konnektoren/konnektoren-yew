use super::{NavItem, NavigationBar, NavigationBarLayout, NavigationHeader};
use yew::prelude::*;
use yew_router::Routable;

/// Header navigation on larger screens and a bottom bar on smaller screens.
#[derive(Properties, PartialEq)]
pub struct ResponsiveNavigationProps<Route: Routable + 'static> {
    pub items: Vec<NavItem<Route>>,
    pub brand: Html,
    #[prop_or_default]
    pub tools: Html,
    #[prop_or_default]
    pub secondary_items: Vec<NavItem<Route>>,
    #[prop_or_else(|| "Main navigation".into())]
    pub label: AttrValue,
    #[prop_or_else(|| "More".into())]
    pub secondary_label: AttrValue,
    #[prop_or_default]
    pub active_route: Option<Route>,
    #[prop_or_default]
    pub content_target: Option<AttrValue>,
    #[prop_or_default]
    pub header_classes: Classes,
    #[prop_or_default]
    pub bottom_classes: Classes,
}

#[function_component(ResponsiveNavigation)]
pub fn responsive_navigation<Route: Routable + 'static>(
    props: &ResponsiveNavigationProps<Route>,
) -> Html {
    html! {
        <>
            <NavigationHeader brand={props.brand.clone()} tools={props.tools.clone()}
                content_target={props.content_target.clone()} classes={props.header_classes.clone()}>
                <NavigationBar<Route> items={props.items.clone()} secondary_items={props.secondary_items.clone()}
                    label={props.label.clone()} secondary_label={props.secondary_label.clone()} active_route={props.active_route.clone()} />
            </NavigationHeader>
            <NavigationBar<Route> layout={NavigationBarLayout::Bottom} items={props.items.clone()}
                secondary_items={props.secondary_items.clone()} label={props.label.clone()}
                secondary_label={props.secondary_label.clone()} active_route={props.active_route.clone()}
                classes={props.bottom_classes.clone()} />
        </>
    }
}
