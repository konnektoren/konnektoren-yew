use crate::providers::use_profile;
use std::sync::Arc;
use yew::prelude::*;
use yew_chat::prelude::{ChatApp, MessageHandler, RequestMessageHandler};

#[derive(Properties, Clone, PartialEq)]
pub struct ChatProps {
    pub api_url: String,
    pub channel: String,
}

#[function_component(ChatComponent)]
pub fn chat(props: &ChatProps) -> Html {
    let profile = use_profile();
    let channel = props.channel.clone();
    let handler = Arc::new(RequestMessageHandler {
        host: props.api_url.clone(),
    }) as Arc<dyn MessageHandler>;

    let expanded = use_state(|| false);

    let on_toggle = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    html! {
        <div class={classes!("chat-content", if *expanded { "chat--expanded" } else { "" })}>
            <div class="chat__bubble" onclick={on_toggle.clone()}>
                <span class="chat__bubble-icon">{"💬"}</span>
                <span class="chat__bubble-text">{"Chat"}</span>
            </div>
            if *expanded {
                <div class="chat__content">
                    <div class="chat__header">
                        <span class="chat__header-title">{props.channel.clone()}</span>
                        <button class="chat__header-close" onclick={on_toggle}>{"×"}</button>
                    </div>
                    <div class="chat__messages">
                        <ChatApp
                            user={profile.name.clone()}
                            channel={channel.clone()}
                            handler={handler.clone()}
                        />
                    </div>
                </div>
            }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChatComponent,
        ChatProps {
            api_url: "https://api.konnektoren.help".to_string(),
            channel: "yew-preview-1".to_string(),
        },
    );
}
