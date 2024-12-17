use crate::model::Inbox;
use chrono::Utc;
use yew::prelude::*;
use yew_chat::prelude::Message;

#[derive(Properties, PartialEq, Clone)]
pub struct InboxProps {
    pub inbox: Inbox,
    pub on_read_message: Callback<String>,
}

#[function_component(InboxComponent)]
pub fn inbox_component(props: &InboxProps) -> Html {
    let is_open = use_state(|| false);

    let unread_count =
        props.inbox.messages.len() - props.inbox.read_messages.as_ref().map_or(0, |v| v.len());

    let toggle_inbox = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let mark_as_read = props.on_read_message.clone();

    html! {
        <div class={classes!("inbox-component", if *is_open { "open" } else { "" })}>
            if *is_open {
                <div class="inbox-content">
                    <button class="close-button" onclick={toggle_inbox.clone()}>{"×"}</button>
                    <h2>{"Inbox"}</h2>
                    <div class="message-list">
                        {for props.inbox.messages.iter().map(|message| {
                            let is_read = props.inbox.read_messages.as_ref()
                                .map(|read| read.contains(&message.id.clone().unwrap_or_default()))
                                .unwrap_or(false);
                            let mark_as_read = mark_as_read.clone();
                            let message_id = message.id.clone().unwrap_or_default();
                            html! {
                                <div
                                    class={classes!("message", if is_read { "read" } else { "unread" })}
                                    onclick={Callback::from(move |_| mark_as_read.emit(message_id.clone()))}
                                >
                                    <div class="message-header">
                                        <span class="sender">{&message.sender}</span>
                                        <span class="timestamp">{message.timestamp.with_timezone(&Utc).format("%Y-%m-%d %H:%M").to_string()}</span>
                                    </div>
                                    <div class="message-content">{&message.content}</div>
                                </div>
                            }
                        })}
                    </div>
                </div>
            }
            <div class="inbox-icon" onclick={toggle_inbox}>
                <i class="fa-solid fa-envelope"></i>
                if unread_count > 0 {
                    <span class="unread-count">{unread_count}</span>
                }
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
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
