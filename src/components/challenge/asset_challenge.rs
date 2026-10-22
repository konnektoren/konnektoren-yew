//! Fetch a challenge dataset on mount, then render the normal challenge UI.
//! Dataset bytes are supplied over HTTP; this module embeds no challenge files.
use super::ChallengeComponent;
use crate::i18n::use_i18n;
use konnektoren_asset_loader::{AssetFormat, AssetServer, UrlSource};
use konnektoren_core::{
    challenges::{Challenge, ChallengeConfig, ChallengeType},
    commands::Command,
    events::Event,
};
use std::{cell::Cell, rc::Rc, sync::Arc};
use yew::prelude::*;

struct ChallengeYaml;
impl AssetFormat for ChallengeYaml {
    type Asset = ChallengeType;
    type Error = serde_yaml::Error;

    fn cache_key(&self) -> Option<String> {
        Some(String::new())
    }

    fn parse(&self, bytes: &[u8]) -> Result<Self::Asset, Self::Error> {
        serde_yaml::from_slice(bytes)
    }
}

enum LoadState {
    Loading,
    Ready {
        asset: Arc<ChallengeType>,
        cache_hit: bool,
    },
    Failed(String),
}

#[derive(Properties, PartialEq)]
pub struct AssetChallengeProps {
    /// Relative to `base_url`, or an absolute HTTP(S) URL.
    pub asset_path: AttrValue,
    #[prop_or_else(|| "/assets/".into())]
    pub base_url: AttrValue,
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

/// Browser-only entry point for datasets that should be fetched on first use.
/// A mounted instance retains its server cache between reloads. Unmounting drops
/// that cache; no global cache or embedded fallback is used.
#[function_component(AssetChallenge)]
pub fn asset_challenge(props: &AssetChallengeProps) -> Html {
    let i18n = use_i18n();
    let state = use_state(|| LoadState::Loading);
    let request = use_state(|| 0usize);
    let server = use_memo(props.base_url.clone(), |base| {
        AssetServer::new(Rc::new(UrlSource::new(base.to_string())))
    });

    {
        let server = server.clone();
        let state = state.clone();
        let path = props.asset_path.to_string();
        use_effect_with(
            (props.base_url.clone(), props.asset_path.clone(), *request),
            move |_| {
                let cancelled = Rc::new(Cell::new(false));
                let task_cancelled = cancelled.clone();
                let previous = match &*state {
                    LoadState::Ready { asset, .. } => Some(asset.clone()),
                    _ => None,
                };
                state.set(LoadState::Loading);
                wasm_bindgen_futures::spawn_local(async move {
                    let result = server.load(ChallengeYaml, path).await;
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
        let server = server.clone();
        let path = props.asset_path.to_string();
        Callback::from(move |_| {
            server.invalidate(&path);
            request.set(*request + 1);
        })
    };
    let loading = matches!(&*state, LoadState::Loading);

    html! {
        <section class="asset-challenge flex flex-col gap-4 p-4">
            if props.show_loader_controls {
                <p><code>{format!("{}{}", props.base_url, props.asset_path)}</code></p>
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
            config: config(),
            language: Some("de".to_owned()),
            on_command: None,
            on_event: None,
            show_loader_controls: true,
        },
        (
            "Missing file",
            AssetChallengeProps {
                asset_path: "challenges/missing.yml".into(),
                base_url: "/assets/".into(),
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
                config: config(),
                language: None,
                on_command: None,
                on_event: None,
                show_loader_controls: true,
            }
        ),
    );
}
