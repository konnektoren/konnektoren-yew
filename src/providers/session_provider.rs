use crate::{
    model::SessionInitializer,
    repository::{SessionRepositoryTrait, SESSION_STORAGE_KEY},
};
use konnektoren_core::session::Session;
use std::sync::Arc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SessionContext {
    pub session: UseStateHandle<Session>,
}

#[derive(Properties)]
pub struct SessionProviderProps {
    pub children: Children,
    pub session_repository: Arc<dyn SessionRepositoryTrait>,
    pub session_initializer: Arc<dyn SessionInitializer>,
}

impl PartialEq for SessionProviderProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.session_repository, &other.session_repository)
            && Arc::ptr_eq(&self.session_initializer, &other.session_initializer)
    }
}

#[function_component(SessionProvider)]
pub fn session_provider(props: &SessionProviderProps) -> Html {
    let session_initializer = props.session_initializer.clone();
    let session = use_state(|| session_initializer.initialize(&Session::default()).unwrap());

    // Load session
    {
        let session = session.clone();
        let session_repository = props.session_repository.clone();
        let session_initializer = session_initializer.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(Some(loaded_session)) =
                    session_repository.get_session(SESSION_STORAGE_KEY).await
                {
                    let initialized_session =
                        session_initializer.initialize(&loaded_session).unwrap();
                    session.set(initialized_session);
                }
            });
            || ()
        });
    }

    {
        let session_repository = props.session_repository.clone();
        let session = session.clone();

        use_effect_with(session.clone(), move |_| {
            let session = session.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let session = session.clone();
                if let Err(e) = session_repository
                    .update_session(SESSION_STORAGE_KEY, &session)
                    .await
                {
                    log::error!("Failed to save session: {:?}", e);
                }
            });
            || ()
        });
    }

    let context = SessionContext { session };

    html! {
        <ContextProvider<SessionContext> {context}>
            { props.children.clone() }
        </ContextProvider<SessionContext>>
    }
}
