use crate::prelude::{ProfilePointsComponent, use_profile};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProfilePointsManagerProps {
    #[prop_or_default]
    pub children: Option<ChildrenWithProps<ProfilePointsComponent>>,
}

#[function_component(ProfilePointsManager)]
pub fn profile_manager(props: &ProfilePointsManagerProps) -> Html {
    let profile_state = use_profile();

    match &props.children {
        Some(children) => {
            let modified_children: Vec<_> = children
                .iter()
                .map(|mut item| {
                    let props = Rc::make_mut(&mut item.props);
                    props.profile = (*profile_state).clone();
                    item
                })
                .collect();
            html! { <>{ for modified_children.into_iter() }</> }
        }
        None => html! {
            <ProfilePointsComponent profile={(*profile_state).clone()} />
        },
    }
}
