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
    let error = use_state(|| None::<String>);

    let session = use_state(
        || match session_initializer.initialize(&Session::default()) {
            Ok(session) => session,
            Err(e) => {
                log::error!("Failed to initialize session: {:?}", e);
                Session::default()
            }
        },
    );

    // Load session
    {
        let session = session.clone();
        let error = error.clone();
        let session_repository = props.session_repository.clone();
        let session_initializer = session_initializer.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match session_repository.get_session(SESSION_STORAGE_KEY).await {
                    Ok(Some(loaded_session)) => {
                        match session_initializer.initialize(&loaded_session) {
                            Ok(initialized_session) => {
                                session.set(initialized_session);
                                error.set(None);
                            }
                            Err(e) => {
                                let err_msg = format!("Failed to initialize session: {:?}", e);
                                log::error!("{}", err_msg);
                                error.set(Some(err_msg));
                            }
                        }
                    }
                    Ok(None) => {
                        log::info!("No existing session found");
                    }
                    Err(e) => {
                        let err_msg = format!("Failed to load session: {:?}", e);
                        log::error!("{}", err_msg);
                        error.set(Some(err_msg));
                    }
                }
            });
            || ()
        });
    }

    {
        let session_repository = props.session_repository.clone();
        let session = session.clone();
        let error = error.clone();

        use_effect_with(session.clone(), move |_| {
            let session = session.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let session = session.clone();
                if let Err(e) = session_repository
                    .update_session(SESSION_STORAGE_KEY, &session)
                    .await
                {
                    let err_msg = format!("Failed to save session: {:?}", e);
                    log::error!("{}", err_msg);
                    error.set(Some(err_msg));
                }
            });
            || ()
        });
    }

    let context = SessionContext { session };

    html! {
        <ContextProvider<SessionContext> {context}>
            if let Some(err_msg) = (*error).clone() {
                <div class="error-message" style="color: red; padding: 1em; margin: 1em; border: 1px solid red;">
                    <h3>{"Session Error"}</h3>
                    <p>{err_msg}</p>
                </div>
            }
            { props.children.clone() }
        </ContextProvider<SessionContext>>
    }
}
