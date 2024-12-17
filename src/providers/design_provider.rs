use yew::prelude::*;

use crate::model::Design;

#[derive(Clone, PartialEq)]
pub struct DesignContext {
    pub design: UseStateHandle<Design>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct DesignProviderProps {
    pub children: Children,
}

#[function_component(DesignProvider)]
pub fn design_provider(props: &DesignProviderProps) -> Html {
    let design = use_state(Design::get_from_body);

    let context = DesignContext { design };

    html! {
        <ContextProvider<DesignContext> context={context}>
            {props.children.clone()}
        </ContextProvider<DesignContext>>
    }
}

#[hook]
pub fn use_design() -> UseStateHandle<Design> {
    use_context::<DesignContext>()
        .expect("use_design must be used within a DesignProvider")
        .design
}
