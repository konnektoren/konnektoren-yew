//! Load a challenge on mount from an injected source, or HTTP by default.
use super::ChallengeComponent;
use crate::{i18n::use_i18n, model::GameLoaderHandle};
use konnektoren_asset_loader::{AssetServer, AssetSource, SourceLayer, UrlSource};
use konnektoren_core::{
    challenges::{Challenge, ChallengeConfig, ChallengeType},
    commands::Command,
    events::Event,
};
use konnektoren_platform::manifest::{Manifest, NoExtensions};
use std::{cell::Cell, rc::Rc, sync::Arc};
use yew::prelude::*;

enum LoadState {
    Loading,
    Ready {
        asset: Arc<ChallengeType>,
        cache_hit: bool,
    },
    Failed(String),
}

/// A source handle whose identity remains stable across Yew renders.
#[derive(Clone)]
pub struct ChallengeSource(Rc<dyn SourceLayer>);

impl ChallengeSource {
    pub fn new(source: impl AssetSource + 'static) -> Self {
        Self(Rc::new(source))
    }
}

impl PartialEq for ChallengeSource {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Properties, PartialEq)]
pub struct AssetChallengeProps {
    /// Relative to `base_url`, or an absolute HTTP(S) URL.
    pub asset_path: AttrValue,
    #[prop_or_else(|| "/assets/".into())]
    pub base_url: AttrValue,
    /// Overrides the default HTTP source. Clone the handle to share its identity.
    #[prop_or_default]
    pub source: Option<ChallengeSource>,
    /// Reuses a platform game loader. Takes precedence over source/base_url.
    #[prop_or_default]
    pub loader: Option<GameLoaderHandle>,
    #[prop_or_default]
    pub config: ChallengeConfig,
    #[prop_or_default]
    pub language: Option<String>,
    #[prop_or_default]
    pub on_command: Option<Callback<Command>>,
    #[prop_or_default]
    pub on_event: Option<Callback<Event>>,
    #[prop_or_default]
    pub show_loader_controls: bool,
}

/// Loads datasets on first use through the platform game loader.
/// Uses HTTP unless a source or shared loader is supplied. Without a shared
/// loader, unmounting drops its cache; an injected handle retains shared assets.
#[function_component(AssetChallenge)]
pub fn asset_challenge(props: &AssetChallengeProps) -> Html {
    let i18n = use_i18n();
    let state = use_state(|| LoadState::Loading);
    let request = use_state(|| 0usize);
    let loader = use_memo(
        (
            props.base_url.clone(),
            props.source.clone(),
            props.loader.clone(),
        ),
        |(base, source, loader)| {
            if let Some(loader) = loader {
                return loader.clone();
            }
            let source = source.as_ref().map_or_else(
                || Rc::new(UrlSource::new(base.to_string())) as Rc<dyn SourceLayer>,
                |source| source.0.clone(),
            );
            GameLoaderHandle::with_server(
                &Manifest::<NoExtensions>::default(),
                AssetServer::new(source),
            )
        },
    );

    {
        let loader = loader.clone();
        let content_id = props.config.challenge.clone();
        let state = state.clone();
        let path = props.asset_path.to_string();
        use_effect_with(
            (
                props.base_url.clone(),
                props.source.clone(),
                props.loader.clone(),
                props.config.challenge.clone(),
                props.asset_path.clone(),
                *request,
            ),
            move |_| {
                let cancelled = Rc::new(Cell::new(false));
                let task_cancelled = cancelled.clone();
                let previous = match &*state {
                    LoadState::Ready { asset, .. } => Some(asset.clone()),
                    _ => None,
                };
                state.set(LoadState::Loading);
                wasm_bindgen_futures::spawn_local(async move {
                    let loader = loader
                        .loader()
                        .clone()
                        .with_challenge_path_resolver(move |_| path.clone());
                    let result = loader.load_challenge(&content_id).await;
                    if !task_cancelled.get() {
                        state.set(match result {
                            Ok(asset) => LoadState::Ready {
                                cache_hit: previous
                                    .as_ref()
                                    .is_some_and(|old| Arc::ptr_eq(old, &asset)),
                                asset,
                            },
                            Err(error) => LoadState::Failed(error.to_string()),
                        });
                    }
                });
                move || cancelled.set(true)
            },
        );
    }

    let reload = {
        let request = request.clone();
        Callback::from(move |_| request.set(*request + 1))
    };
    let clear_and_reload = {
        let request = request.clone();
        let loader = loader.clone();
        let path = props.asset_path.to_string();
        Callback::from(move |_| {
            loader.loader().server().invalidate(&path);
            request.set(*request + 1);
        })
    };
    let loading = matches!(&*state, LoadState::Loading);

    html! {
        <section class="asset-challenge flex flex-col gap-4 p-4">
            if props.show_loader_controls {
                <p><code>{props.asset_path.clone()}</code></p>
                <div class="flex flex-wrap gap-2">
                    <button class="btn btn-primary" onclick={reload.clone()} disabled={loading}>
                        {i18n.t("Load again")}
                    </button>
                    <button class="btn btn-secondary" onclick={clear_and_reload} disabled={loading}>
                        {i18n.t("Clear cache and reload")}
                    </button>
                </div>
            }
            <div role="status" aria-live="polite">
                {match &*state {
                    LoadState::Loading => html! { <p>{i18n.t("Loading...")}</p> },
                    LoadState::Failed(error) => html! {
                        <div class="flex flex-col gap-2">
                            <p class="text-error">{i18n.t("Could not load challenge.")}</p>
                            if props.show_loader_controls { <p>{error}</p> }
                            <button class="btn btn-primary" onclick={reload}>{i18n.t("Retry")}</button>
                        </div>
                    },
                    LoadState::Ready { asset, cache_hit } => html! {
                        <>
                            if props.show_loader_controls {
                                <p>{if *cache_hit { i18n.t("Cache hit") } else { i18n.t("Loaded from source") }}</p>
                            }
                            <ChallengeComponent
                                key={format!("{}:{}:{}", props.base_url, props.asset_path, *request)}
                                challenge={Challenge::new(&asset.of_tasks(&props.config.tasks), &props.config)}
                                variant={props.config.variant.clone()}
                                language={props.language.clone()}
                                on_command={props.on_command.clone()}
                                on_event={props.on_event.clone()}
                                preview={props.show_loader_controls}
                            />
                        </>
                    },
                }}
            </div>
        </section>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use crate::model::preview_game_assets;
    use yew_preview::prelude::*;

    fn config() -> ChallengeConfig {
        ChallengeConfig {
            id: "multiple-choice-preview".to_owned(),
            name: "Multiple Choice".to_owned(),
            description: String::new(),
            challenge: "multiple-choice".to_owned(),
            tasks: 3.into(),
            ..Default::default()
        }
    }

    yew_preview::create_preview!(
        AssetChallenge,
        AssetChallengeProps {
            asset_path: "challenges/multiple_choice.yml".into(),
            base_url: "/assets/".into(),
            source: None,
            loader: preview_game_assets::loader().ok(),
            config: config(),
            language: Some("de".to_owned()),
            on_command: None,
            on_event: None,
            show_loader_controls: true,
        },
        (
            "Embedded",
            AssetChallengeProps {
                asset_path: "challenges/konnektoren.yml".into(),
                base_url: "/assets/".into(),
                source: None,
                loader: preview_game_assets::loader().ok(),
                config: ChallengeConfig {
                    id: "embedded-preview".to_owned(),
                    name: "Konnektoren".to_owned(),
                    description: String::new(),
                    challenge: "konnektoren".to_owned(),
                    tasks: 3.into(),
                    ..Default::default()
                },
                language: Some("de".to_owned()),
                on_command: None,
                on_event: None,
                show_loader_controls: true,
            }
        ),
        (
            "Missing file",
            AssetChallengeProps {
                asset_path: "challenges/missing.yml".into(),
                base_url: "/assets/".into(),
                source: None,
                loader: preview_game_assets::loader().ok(),
                config: config(),
                language: None,
                on_command: None,
                on_event: None,
                show_loader_controls: true,
            }
        ),
        (
            "Invalid dataset",
            AssetChallengeProps {
                asset_path: "inbox.yml".into(),
                base_url: "/assets/".into(),
                source: None,
                loader: preview_game_assets::loader().ok(),
                config: config(),
                language: None,
                on_command: None,
                on_event: None,
                show_loader_controls: true,
            }
        ),
    );
}
