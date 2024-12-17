use crate::components::InboxComponent;
use crate::prelude::use_inbox;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InboxManagerProps {
    #[prop_or_default]
    pub children: Option<ChildrenWithProps<InboxComponent>>,
}

#[function_component(InboxManager)]
pub fn inbox_manager(props: &InboxManagerProps) -> Html {
    let inbox_state = use_inbox();

    let mark_as_read = {
        let inbox_state = inbox_state.clone();
        Callback::from(move |message_id: String| {
            let mut current_inbox = (*inbox_state).clone();
            let read_messages = current_inbox.read_messages.get_or_insert_with(Vec::new);
            if !read_messages.contains(&message_id) {
                read_messages.push(message_id);
                inbox_state.set(current_inbox.clone());
            }
        })
    };

    match &props.children {
        Some(children) => {
            let modified_children = children.iter().map(|mut item| {
                let props = Rc::make_mut(&mut item.props);
                props.inbox = (&*inbox_state).clone();
                props.on_read_message = mark_as_read.clone();
                item
            });
            html! { for modified_children }
        }
        None => {
            return html! {
                <InboxComponent inbox={(&*inbox_state).clone()} on_read_message={mark_as_read.clone()} />
            }
        }
    }
}
