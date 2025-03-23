#[cfg(feature = "csr")]
use crate::app_csr::AppCSR;
#[cfg(feature = "ssr")]
use crate::app_ssr::AppSSR;

use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    #[cfg(feature = "csr")]
    html! {
        <>
            <AppCSR />
        </>
    }
    #[cfg(feature = "ssr")]
    html! {
        <>
            <AppSSR />
        </>
    }
    #[cfg(not(any(feature = "csr", feature = "ssr")))]
    html! {}
}
