use super::repository_hooks::{use_session, use_session_repository};
use crate::repository::GameStatePersistenceImpl;
use konnektoren_core::commands::CommandBus;
use konnektoren_core::controller::{
    ChallengeFinishPlugin, GameController, GameControllerTrait, GameXpPlugin,
};
use konnektoren_core::events::EventBus;
use konnektoren_core::game::{Game, GameState};
use std::sync::{Arc, RwLock};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct GameControllerContext {
    pub controller: Arc<GameController>,
}

impl GameControllerContext {
    pub fn new(controller: Arc<GameController>) -> Self {
        Self { controller }
    }
}

#[hook]
pub fn use_game_controller() -> GameControllerContext {
    use_context::<GameControllerContext>().expect("GameControllerContext not found")
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
        log::error!("Failed to load game state: {:?}", e);
    }

    let context = GameControllerContext::new(controller.clone());

    {
        let session = session.clone();
        let controller = controller.clone();
        use_effect_with(session, move |_| {
            if let Err(e) = controller.load_game_state() {
                log::error!("Failed to reload game state: {:?}", e);
            }
        })
    }

    html! {
        <ContextProvider<GameControllerContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<GameControllerContext>>
    }
}
