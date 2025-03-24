use crate::i18n::create_i18n_config;
use crate::model::DefaultSessionInitializer;
use crate::providers::create_repositories;
use crate::providers::{
    DesignProvider, GameControllerProvider, I18nProvider, RepositoryProvider, ThemeProvider,
};
use crate::repository::LocalStorage;
use crate::route::Route;
use crate::switch_route::switch_route;
use std::sync::Arc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WrappedAppProps {
    pub route: Route,
}

#[function_component(WrappedApp)]
pub fn wrapped_app(props: &WrappedAppProps) -> Html {
    // Setup repositories and providers
    let i18n_config = create_i18n_config();
    let storage = LocalStorage::new(None);
    let session_initilizer = DefaultSessionInitializer;
    let repository_config = create_repositories(storage, Arc::new(session_initilizer));

    let route = props.route.clone();

    html! {
        <RepositoryProvider config={repository_config}>
            <ThemeProvider>
                <DesignProvider>
                    <I18nProvider config={i18n_config}>
                        <GameControllerProvider>
                            {switch_route(route)}
                        </GameControllerProvider>
                    </I18nProvider>
                </DesignProvider>
            </ThemeProvider>
        </RepositoryProvider>
    }
}
