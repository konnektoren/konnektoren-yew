use crate::{
    model::SessionInitializer,
    repository::{SESSION_STORAGE_KEY, SessionRepositoryTrait},
};
use konnektoren_core::challenges::{ChallengeType, SavedChallenge};
use konnektoren_core::session::Session;
use std::sync::Arc;
use yew::prelude::*;

/// The player's session. Applications with their own challenge type provide a
/// `SessionContext<TheirType>` through `SessionProvider<TheirType>`.
pub struct SessionContext<C: SavedChallenge = ChallengeType> {
    pub session: UseStateHandle<Session<C>>,
}

impl<C: SavedChallenge> Clone for SessionContext<C> {
    fn clone(&self) -> Self {
        Self {
            session: self.session.clone(),
        }
    }
}

impl<C: SavedChallenge> PartialEq for SessionContext<C> {
    fn eq(&self, other: &Self) -> bool {
        self.session == other.session
    }
}

#[cfg(feature = "csr")]
fn should_persist_session(is_hydrated: bool) -> bool {
    is_hydrated
}

#[derive(Properties)]
pub struct SessionProviderProps<C: SavedChallenge = ChallengeType> {
    pub children: Children,
    pub session_repository: Arc<dyn SessionRepositoryTrait<C>>,
    pub session_initializer: Arc<dyn SessionInitializer<C>>,
    /// The session to start from before a saved one is loaded, and when loading fails.
    pub initial_session: Session<C>,
}

impl<C: SavedChallenge> PartialEq for SessionProviderProps<C> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.session_repository, &other.session_repository)
            && Arc::ptr_eq(&self.session_initializer, &other.session_initializer)
            && self.initial_session == other.initial_session
    }
}

#[function_component(SessionProvider)]
pub fn session_provider<C: SavedChallenge>(props: &SessionProviderProps<C>) -> Html {
    let session_initializer = props.session_initializer.clone();
    let initial_session = props.initial_session.clone();
    let error = use_state(|| None::<String>);
    #[cfg(feature = "csr")]
    let is_hydrated = use_state(|| false);

    let session = use_state(|| match session_initializer.initialize(&initial_session) {
        Ok(session) => session,
        Err(e) => {
            tracing::error!("Failed to initialize session: {:?}", e);
            initial_session.clone()
        }
    });

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
                                tracing::error!("{}", err_msg);
                                error.set(Some(err_msg));
                            }
                        }
                    }
                    Ok(None) => {
                        tracing::info!("No existing session found");
                        is_hydrated.set(true);
                    }
                    Err(e) => {
                        let err_msg = format!("Failed to load session: {:?}", e);
                        tracing::error!("{}", err_msg);
                        error.set(Some(err_msg));
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
                    tracing::error!("{}", err_msg);
                    error.set(Some(err_msg));
                }
            });
            || ()
        });
    }

    let context = SessionContext::<C> { session };

    #[cfg(feature = "csr")]
    let ready = *is_hydrated;
    #[cfg(not(feature = "csr"))]
    let ready = true;

    html! {
        <ContextProvider<SessionContext<C>> {context}>
            if let Some(err_msg) = (*error).clone() {
                <div class="error-message" style="color: red; padding: 1em; margin: 1em; border: 1px solid red;">
                    <h3>{"Session Error"}</h3>
                    <p>{err_msg}</p>
                </div>
            }
            if ready {
                { props.children.clone() }
            }
        </ContextProvider<SessionContext<C>>>
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
