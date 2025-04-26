use yew::Html;

#[derive(Clone, PartialEq)]
pub enum NavItem<Route> {
    Route {
        name: &'static str,
        route: Route,
        icon: &'static str,
    },
    Component {
        component: Html,
    },
    Group {
        name: &'static str,
        icon: &'static str,
        items: Vec<NavItem<Route>>,
    },
}
