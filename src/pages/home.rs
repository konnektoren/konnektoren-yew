use crate::app_csr::AppCSR;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <AppCSR />
        </>
    }
}
