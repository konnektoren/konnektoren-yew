use yew::prelude::*;

#[function_component]
pub fn ClientSideRouter(props: &RouterProps) -> Html {
    #[cfg(feature = "csr")]
    {
        html! {
            <yew_router::prelude::BrowserRouter>
                { props.children.clone() }
            </yew_router::prelude::BrowserRouter>
        }
    }

    #[cfg(not(feature = "csr"))]
    {
        html! {
            { props.children.clone() }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct RouterProps {
    #[prop_or_default]
    pub children: Children,
}
