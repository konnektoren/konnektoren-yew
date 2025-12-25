use crate::model::Inbox;
use chrono::Utc;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InboxProps {
    pub inbox: Inbox,
    pub on_read_message: Callback<String>,
}

#[function_component(InboxComponent)]
pub fn inbox_component(props: &InboxProps) -> Html {
    let is_open = use_state(|| false);

    #[cfg(feature = "csr")]
    let inbox_ref = use_node_ref();

    #[cfg(not(feature = "csr"))]
    let inbox_ref = NodeRef::default();

    let unread_count = props
        .inbox
        .messages
        .len()
        .saturating_sub(props.inbox.read_messages.as_ref().map_or(0, |v| v.len()));

    let toggle_inbox = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let mark_as_read = props.on_read_message.clone();

    // Handle outside click
    #[cfg(feature = "csr")]
    {
        use wasm_bindgen::JsCast;
        use yew::use_effect_with;

        let handle_outside_click = {
            let is_open = is_open.clone();
            let inbox_ref = inbox_ref.clone();

            Callback::from(move |e: MouseEvent| {
                if let Some(inbox_element) = inbox_ref.cast::<web_sys::HtmlElement>() {
                    let target = e.target();
                    if let Some(target_element) =
                        target.and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                    {
                        if !inbox_element.contains(Some(&target_element)) {
                            is_open.set(false);
                        }
                    }
                }
            })
        };

        use_effect_with(is_open.clone(), move |is_open| {
            let cleanup_fn = if **is_open {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let handle_outside_click_clone = handle_outside_click.clone();

                let listener = gloo::events::EventListener::new(&document, "mousedown", move |e| {
                    let event = e.clone().dyn_into::<web_sys::MouseEvent>().unwrap();
                    handle_outside_click_clone.emit(event);
                });

                Box::new(move || drop(listener)) as Box<dyn FnOnce()>
            } else {
                Box::new(|| {}) as Box<dyn FnOnce()>
            };

            cleanup_fn
        });
    }

    html! {
        <div class="inbox-component">
            if *is_open {
                <div class="inbox-component__content" ref={inbox_ref}>
                    <div class="inbox-component__header">
                        <h2 class="inbox-component__title">{"Inbox"}</h2>
                        <button
                            class="inbox-component__close"
                            onclick={toggle_inbox.clone()}
                            aria-label="Close inbox"
                        >
                            {"×"}
                        </button>
                    </div>
                    <div class="inbox-component__messages">
                        if props.inbox.messages.is_empty() {
                            <div class="inbox-component__empty">
                                <i class="fa-solid fa-inbox text-4xl mb-4"></i>
                                <p>{"No messages yet"}</p>
                            </div>
                        } else {
                            {for props.inbox.messages.iter().map(|message| {
                                let is_read = props.inbox.read_messages.as_ref()
                                    .map(|read| read.contains(&message.id.clone().unwrap_or_default()))
                                    .unwrap_or(false);
                                let mark_as_read = mark_as_read.clone();
                                let message_id = message.id.clone().unwrap_or_default();

                                html! {
                                    <div
                                        class={classes!(
                                            "inbox-component__message",
                                            (!is_read).then_some("inbox-component__message--unread")
                                        )}
                                        onclick={Callback::from(move |_| mark_as_read.emit(message_id.clone()))}
                                    >
                                        <div class="inbox-component__message-header">
                                            <span class="inbox-component__message-sender">{&message.sender}</span>
                                            <span class="inbox-component__message-timestamp">
                                                {message.timestamp.with_timezone(&Utc).format("%Y-%m-%d %H:%M").to_string()}
                                            </span>
                                        </div>
                                        <div class="inbox-component__message-content">{&message.content}</div>
                                    </div>
                                }
                            })}
                        }
                    </div>
                </div>
            }
            <div class="inbox-component__icon" onclick={toggle_inbox}>
                <i class="fa-solid fa-envelope"></i>
                if unread_count > 0 {
                    <span class="inbox-component__unread-count">{unread_count}</span>
                }
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_chat::prelude::Message;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        InboxComponent,
        InboxProps {
            inbox: Inbox {
                messages: vec![
                    Message {
                        id: Some("1".to_string()),
                        sender: "System".to_string(),
                        content: "Welcome to the game!".to_string(),
                        timestamp: Utc::now(),
                    },
                    Message {
                        id: Some("2".to_string()),
                        sender: "Admin".to_string(),
                        content: "New update available".to_string(),
                        timestamp: Utc::now(),
                    },
                ],
                read_messages: Some(vec!["1".to_string()]),
            },
            on_read_message: Callback::from(|_| ()),
        },
    );
}
