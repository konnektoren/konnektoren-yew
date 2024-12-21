use super::NavExtra;
use super::NavItem;

#[derive(Clone, PartialEq)]
pub struct NavGroup<Route> {
    pub name: &'static str,
    pub icon: &'static str,
    pub items: Vec<NavItem<Route>>,
    pub extras: Option<Vec<NavExtra>>,
}
