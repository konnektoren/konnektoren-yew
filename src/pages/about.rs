use crate::components::{AppDependenciesComponent, AppVersionComponent};
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>
        <AppDependenciesComponent />
        <AppVersionComponent />
        </div>
    }
}
