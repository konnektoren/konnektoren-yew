use crate::model::Inbox;
use crate::repository::{INBOX_STORAGE_KEY, InboxRepositoryTrait};
use std::sync::Arc;
use yew::prelude::*;

const INBOX_FILE: &str = "/assets/inbox.yml";

#[derive(Clone, PartialEq)]
pub struct InboxContext {
    pub inbox: UseStateHandle<Inbox>,
}

#[derive(Properties)]
pub struct InboxProviderProps {
    pub children: Children,
    pub inbox_repository: Arc<dyn InboxRepositoryTrait>,
}

impl PartialEq for InboxProviderProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inbox_repository, &other.inbox_repository)
    }
}

#[function_component(InboxProvider)]
pub fn inbox_provider(props: &InboxProviderProps) -> Html {
    let inbox = use_state(Inbox::default);

    // Load inbox
    {
        let inbox = inbox.clone();
        let inbox_repository = props.inbox_repository.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(Some(loaded_inbox)) = inbox_repository.get_inbox(INBOX_STORAGE_KEY).await
                {
                    log::info!("Loaded inbox: {:?}", loaded_inbox);
                    inbox.set(loaded_inbox.clone());
                    match gloo::net::http::Request::get(INBOX_FILE).send().await {
                        Ok(response) => match response.text().await {
                            Ok(text) => {
                                let mut new_inbox = loaded_inbox.clone();
                                let loaded_inbox: Inbox = serde_yaml::from_str(&text)
                                    .unwrap_or_else(|e| {
                                        log::error!("Failed to parse inbox YAML: {:?}", e);
                                        Inbox::default()
                                    });
                                new_inbox.merge(&loaded_inbox);
                                inbox.set(new_inbox);
                            }
                            Err(e) => log::error!("Failed to get response text: {:?}", e),
                        },
                        Err(e) => log::error!("Failed to load inbox: {:?}", e),
                    }
                }
            });
            || ()
        });
    }

    {
        let inbox_repository = props.inbox_repository.clone();
        let inbox = inbox.clone();

        use_effect_with(inbox.clone(), move |_| {
            let inbox = inbox.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let inbox = inbox.clone();
                if let Err(e) = inbox_repository.save_inbox(INBOX_STORAGE_KEY, &inbox).await {
                    log::error!("Failed to save inbox: {:?}", e);
                }
            });
            || ()
        });
    }

    let context = InboxContext { inbox };

    html! {
        <ContextProvider<InboxContext> {context}>
            { for props.children.iter() }
        </ContextProvider<InboxContext>>
    }
}
