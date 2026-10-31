use super::repository_hooks::{use_session, use_session_of, use_session_repository};
use crate::repository::{GameStatePersistenceImpl, SessionRepositoryTrait};
use konnektoren_core::challenges::{
    ChallengeEmbedding, ChallengeType, GameChallenge, SavedChallenge,
};
use konnektoren_core::commands::{Command, CommandBus};
use konnektoren_core::controller::{
    ChallengeFinishPlugin, GameController, GameControllerTrait, GameXpPlugin,
};
use konnektoren_core::events::EventBus;
use konnektoren_core::game::{Game, GameState};
use std::sync::{Arc, RwLock};
use yew::prelude::*;

/// The running game controller. Applications that extend the built-in challenges with
/// their own type provide a `GameControllerContext<TheirType>`.
pub struct GameControllerContext<C: GameChallenge = ChallengeType> {
    pub controller: Arc<GameController<C>>,
}

impl<C: GameChallenge> Clone for GameControllerContext<C> {
    fn clone(&self) -> Self {
        Self {
            controller: self.controller.clone(),
        }
    }
}

impl<C: GameChallenge> PartialEq for GameControllerContext<C> {
    fn eq(&self, other: &Self) -> bool {
        self.controller == other.controller
    }
}

impl<C: GameChallenge> GameControllerContext<C> {
    pub fn new(controller: Arc<GameController<C>>) -> Self {
        Self { controller }
    }
}

#[hook]
pub fn use_game_controller() -> GameControllerContext {
    use_context::<GameControllerContext>().expect("GameControllerContext not found")
}

/// Like [`use_game_controller`] for an application's own challenge type.
#[hook]
pub fn use_game_controller_of<C: GameChallenge>() -> GameControllerContext<C> {
    use_context::<GameControllerContext<C>>().expect("GameControllerContext not found")
}

/// Turns a callback for the application's commands into one that the default components
/// of an included challenge type can emit to. Their commands are embedded first.
pub fn embed_command_callback<Inner: GameChallenge, Outer: GameChallenge>(
    callback: Callback<Command<Outer>>,
    embedding: ChallengeEmbedding<Inner, Outer>,
) -> Callback<Command<Inner>> {
    Callback::from(move |command: Command<Inner>| callback.emit(command.embed(&embedding)))
}

#[hook]
pub fn use_game_state() -> UseStateHandle<GameState> {
    let session = use_session();
    let game_state = use_state(|| session.game_state.clone());

    {
        let session = session.clone();
        let game_state = game_state.clone();
        use_effect_with(session.clone(), move |session| {
            game_state.set(session.game_state.clone());
        });
    }

    // Update session when game state changes
    {
        let session = session.clone();
        let game_state = game_state.clone();
        use_effect_with(game_state.clone(), move |game_state| {
            let mut new_session = (*session).clone();
            new_session.game_state = (**game_state).clone();
            session.set(new_session);
        });
    }

    game_state
}

/// Like [`use_game_state`] for an application's own challenge type.
#[hook]
pub fn use_game_state_of<C: SavedChallenge>() -> UseStateHandle<GameState<C>> {
    let session = use_session_of::<C>();
    let game_state = use_state(|| session.game_state.clone());

    {
        let session = session.clone();
        let game_state = game_state.clone();
        use_effect_with(session.clone(), move |session| {
            game_state.set(session.game_state.clone());
        });
    }

    {
        let session = session.clone();
        let game_state = game_state.clone();
        use_effect_with(game_state.clone(), move |game_state| {
            let mut new_session = (*session).clone();
            new_session.game_state = (**game_state).clone();
            session.set(new_session);
        });
    }

    game_state
}

#[hook]
pub fn use_event_bus() -> Arc<EventBus> {
    let ctx = use_game_controller();
    Arc::new(ctx.controller.event_bus().clone())
}

#[hook]
pub fn use_command_bus() -> Arc<CommandBus> {
    let ctx = use_game_controller();
    Arc::new(ctx.controller.command_bus().clone())
}

#[derive(Properties, PartialEq)]
pub struct GameControllerProviderProps {
    pub children: Children,
    #[prop_or_default]
    pub game_controller: Option<Arc<GameController>>,
}

#[function_component(GameControllerProvider)]
pub fn game_controller_provider(props: &GameControllerProviderProps) -> Html {
    let session_repository = use_session_repository();
    let session = use_session();

    let controller: Arc<GameController> = match &props.game_controller {
        Some(controller) => controller.clone(),
        None => {
            let game = Game::default();
            let session = Arc::new(RwLock::new((*session).clone()));

            let persistence = Arc::new(GameStatePersistenceImpl::new(session_repository, session));

            let mut controller = GameController::new(game, persistence);
            controller.register_plugin(Arc::new(ChallengeFinishPlugin));
            controller.register_plugin(Arc::new(GameXpPlugin));
            controller.init()
        }
    };

    // Load game state with error handling
    if let Err(e) = controller.load_game_state() {
        tracing::error!("Failed to load game state: {:?}", e);
    }

    let context = GameControllerContext::new(controller.clone());

    {
        let session = session.clone();
        let controller = controller.clone();
        use_effect_with(session, move |_| {
            if let Err(e) = controller.load_game_state() {
                tracing::error!("Failed to reload game state: {:?}", e);
            }
        })
    }

    html! {
        <ContextProvider<GameControllerContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<GameControllerContext>>
    }
}

#[derive(Properties)]
pub struct ExtendedGameControllerProviderProps<C: SavedChallenge> {
    pub children: Children,
    /// The application's game, used when the saved session has no state yet.
    pub game: Game<C>,
    pub session_repository: Arc<dyn SessionRepositoryTrait<C>>,
}

impl<C: SavedChallenge> PartialEq for ExtendedGameControllerProviderProps<C> {
    fn eq(&self, other: &Self) -> bool {
        self.game == other.game && Arc::ptr_eq(&self.session_repository, &other.session_repository)
    }
}

/// Provides a `GameControllerContext<C>` for an application's own challenge type, inside a
/// `SessionProvider<C>`. The controller is created once and follows later session changes.
#[function_component(ExtendedGameControllerProvider)]
pub fn extended_game_controller_provider<C: SavedChallenge>(
    props: &ExtendedGameControllerProviderProps<C>,
) -> Html {
    let session = use_session_of::<C>();

    let controller = {
        let session = session.clone();
        let game = props.game.clone();
        let session_repository = props.session_repository.clone();
        use_memo((), move |_| {
            let session = Arc::new(RwLock::new((*session).clone()));
            let persistence = Arc::new(GameStatePersistenceImpl::new(
                session_repository,
                session.clone(),
            ));
            let mut controller = GameController::new(game, persistence);
            controller.register_plugin(Arc::new(ChallengeFinishPlugin));
            controller.register_plugin(Arc::new(GameXpPlugin));
            (controller.init(), session)
        })
    };

    let last_context_state = use_mut_ref(|| None::<GameState<C>>);
    {
        let (controller, stored_session) = &*controller;
        let controller = controller.clone();
        let stored_session = stored_session.clone();
        let last_context_state = last_context_state.clone();
        use_effect_with(session, move |session| {
            let context_changed = last_context_state.borrow().as_ref() != Some(&session.game_state);
            *last_context_state.borrow_mut() = Some(session.game_state.clone());
            match stored_session.write() {
                Ok(mut stored) => {
                    stored.id = session.id.clone();
                    stored.player_profile = session.player_profile.clone();
                    if context_changed {
                        stored.game_state = session.game_state.clone();
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to update controller session: {:?}", e);
                    return;
                }
            }
            if context_changed && let Err(e) = controller.load_game_state() {
                tracing::error!("Failed to load game state: {:?}", e);
            }
        });
    }

    let context = GameControllerContext::new(controller.0.clone());
    html! {
        <ContextProvider<GameControllerContext<C>> {context}>
            { for props.children.iter() }
        </ContextProvider<GameControllerContext<C>>>
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use crate::model::DefaultSessionInitializer;
    use crate::providers::SessionProvider;
    use crate::repository::{MemoryStorage, SessionRepository};
    use konnektoren_core::session::Session;

    #[function_component(Consumer)]
    fn consumer() -> Html {
        let controller = use_game_controller_of::<ChallengeType>().controller;
        let state = controller.game_state().lock().unwrap();
        html! { <p class="current">{state.challenge.challenge_config.id.clone()}</p> }
    }

    #[function_component(Host)]
    fn host() -> Html {
        let repository: Arc<dyn SessionRepositoryTrait> =
            Arc::new(SessionRepository::new(MemoryStorage::default()));
        html! {
            <SessionProvider<ChallengeType>
                session_repository={repository.clone()}
                session_initializer={Arc::new(DefaultSessionInitializer) as Arc<dyn crate::model::SessionInitializer>}
                initial_session={Session::default()}>
                <ExtendedGameControllerProvider<ChallengeType>
                    game={Game::default()} session_repository={repository}>
                    <Consumer />
                </ExtendedGameControllerProvider<ChallengeType>>
            </SessionProvider<ChallengeType>>
        }
    }

    #[tokio::test]
    async fn extended_providers_supply_a_controller_for_the_session() {
        let html = yew::ServerRenderer::<Host>::new()
            .hydratable(false)
            .render()
            .await;
        assert!(html.contains("konnektoren-1"), "{html}");
    }
}
