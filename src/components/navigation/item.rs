#[derive(Clone, PartialEq)]
pub struct NavItem<Route> {
    pub name: &'static str,
    pub route: Route,
    pub icon: &'static str,
}
