use crate::{
    model::SessionInitializer,
    repository::{SESSION_STORAGE_KEY, SessionRepositoryTrait},
};
use konnektoren_core::session::Session;
use std::sync::Arc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SessionContext {
    pub session: UseStateHandle<Session>,
}

#[cfg(feature = "csr")]
fn should_persist_session(is_hydrated: bool) -> bool {
    is_hydrated
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
    #[cfg(feature = "csr")]
    let is_hydrated = use_state(|| false);

    let session = use_state(
        || match session_initializer.initialize(&Session::default()) {
            Ok(session) => session,
            Err(e) => {
                log::error!("Failed to initialize session: {:?}", e);
                Session::default()
            }
        },
    );

    // Load session (CSR only)
    #[cfg(feature = "csr")]
    {
        let session = session.clone();
        let error = error.clone();
        let session_repository = props.session_repository.clone();
        let session_initializer = session_initializer.clone();
        let is_hydrated = is_hydrated.clone();

        use_effect_with((), move |_| {
            use wasm_bindgen_futures::spawn_local;

            spawn_local(async move {
                match session_repository.get_session(SESSION_STORAGE_KEY).await {
                    Ok(Some(loaded_session)) => {
                        match session_initializer.initialize(&loaded_session) {
                            Ok(initialized_session) => {
                                session.set(initialized_session);
                                error.set(None);
                                is_hydrated.set(true);
                            }
                            Err(e) => {
                                let err_msg = format!("Failed to initialize session: {:?}", e);
                                log::error!("{}", err_msg);
                                error.set(Some(err_msg));
                                is_hydrated.set(true);
                            }
                        }
                    }
                    Ok(None) => {
                        log::info!("No existing session found");
                        is_hydrated.set(true);
                    }
                    Err(e) => {
                        let err_msg = format!("Failed to load session: {:?}", e);
                        log::error!("{}", err_msg);
                        error.set(Some(err_msg));
                        is_hydrated.set(true);
                    }
                }
            });
            || ()
        });
    }

    // Save session (CSR only)
    #[cfg(feature = "csr")]
    {
        let session_repository = props.session_repository.clone();
        let session = session.clone();
        let error = error.clone();
        let is_hydrated = is_hydrated.clone();

        use_effect_with((session.clone(), *is_hydrated), move |_| {
            use wasm_bindgen_futures::spawn_local;

            let session = session.clone();
            let is_hydrated = *is_hydrated;
            spawn_local(async move {
                if !should_persist_session(is_hydrated) {
                    return;
                }

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

#[cfg(test)]
mod tests {
    #[cfg(feature = "csr")]
    use super::should_persist_session;

    #[cfg(feature = "csr")]
    #[test]
    fn session_is_not_persisted_before_hydration() {
        assert!(!should_persist_session(false));
        assert!(should_persist_session(true));
    }
}
