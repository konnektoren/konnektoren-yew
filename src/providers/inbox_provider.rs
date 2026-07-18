use crate::model::Inbox;
use crate::repository::{INBOX_STORAGE_KEY, InboxRepositoryTrait};
#[cfg(feature = "csr")]
use konnektoren_asset_loader::AssetServer;
#[cfg(feature = "csr")]
use konnektoren_core::assets::{AssetFormat, DefaultAssetSource};
#[cfg(feature = "csr")]
use std::rc::Rc;
use std::sync::Arc;
use yew::prelude::*;

#[cfg(feature = "csr")]
const INBOX_FILE: &str = "inbox.yml";

#[cfg(feature = "csr")]
#[derive(Clone)]
struct InboxFormat;

#[cfg(feature = "csr")]
impl AssetFormat for InboxFormat {
    type Asset = Inbox;
    type Error = serde_yaml::Error;

    fn extensions(&self) -> &[&str] {
        &["yml", "yaml"]
    }

    fn parse(&self, bytes: &[u8]) -> Result<Self::Asset, Self::Error> {
        serde_yaml::from_slice(bytes)
    }
}

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

    // Load inbox - CSR only
    #[cfg(feature = "csr")]
    {
        let inbox = inbox.clone();
        let inbox_repository = props.inbox_repository.clone();
        let asset_server = use_state(|| AssetServer::new(Rc::new(DefaultAssetSource::default())));

        use_effect_with((), move |_| {
            use wasm_bindgen_futures::spawn_local;

            spawn_local(async move {
                let stored_inbox = match inbox_repository.get_inbox(INBOX_STORAGE_KEY).await {
                    Ok(Some(stored_inbox)) => stored_inbox,
                    Ok(None) => Inbox::default(),
                    Err(error) => {
                        tracing::error!("Failed to load stored inbox: {:?}", error);
                        Inbox::default()
                    }
                };
                inbox.set(stored_inbox.clone());

                match asset_server.load(InboxFormat, INBOX_FILE).await {
                    Ok(remote_inbox) => {
                        let mut merged_inbox = stored_inbox;
                        merged_inbox.merge(&remote_inbox);
                        inbox.set(merged_inbox);
                    }
                    Err(error) => tracing::error!("Failed to load inbox asset: {}", error),
                }
            });
            || ()
        });
    }

    // Save inbox - CSR only
    #[cfg(feature = "csr")]
    {
        let inbox_repository = props.inbox_repository.clone();
        let inbox = inbox.clone();

        use_effect_with(inbox.clone(), move |_| {
            use wasm_bindgen_futures::spawn_local;

            let inbox = inbox.clone();
            spawn_local(async move {
                let inbox = inbox.clone();
                if let Err(e) = inbox_repository.save_inbox(INBOX_STORAGE_KEY, &inbox).await {
                    tracing::error!("Failed to save inbox: {:?}", e);
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
