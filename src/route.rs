use strum_macros::{EnumIter, IntoStaticStr};
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug, EnumIter, IntoStaticStr)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/home/")]
    Home,
    #[at("/example/")]
    Example,
    #[at("/about/")]
    About,
}
