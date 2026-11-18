use yew::Html;

#[derive(Clone, PartialEq)]
pub enum NavItem<Route> {
    Route {
        name: &'static str,
        route: Route,
        icon: Html,
    },
    Component {
        component: Html,
    },
    Group {
        name: &'static str,
        icon: Html,
        items: Vec<NavItem<Route>>,
    },
}
