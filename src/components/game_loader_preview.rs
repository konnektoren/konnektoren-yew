use crate::{components::AssetChallenge, i18n::use_i18n, model::preview_game_assets};
use konnektoren_core::{challenges::ChallengeConfig, game::GamePath};
use std::cell::Cell;
use std::rc::Rc;
use yew::prelude::*;

/// Demonstrates manifest metadata, lazy datasets and explicit whole-game preload.
#[function_component(GameLoaderPreview)]
pub fn game_loader_preview_component() -> Html {
    let i18n = use_i18n();
    let loader = use_memo((), |_| preview_game_assets::loader());
    let levels = use_state(|| None::<Result<Vec<GamePath>, String>>);
    let selected = use_state(|| None::<ChallengeConfig>);
    let preload = use_state(|| None::<Result<(usize, usize), String>>);
    let preloading = use_state(|| false);

    {
        let levels = levels.clone();
        use_effect_with(loader.clone(), move |loader| {
            let cancelled = Rc::new(Cell::new(false));
            let task_cancelled = cancelled.clone();
            let loader = (**loader).clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = match loader {
                    Ok(loader) => loader
                        .loader()
                        .load_game_paths()
                        .await
                        .map_err(|error| error.to_string()),
                    Err(error) => Err(error),
                };
                if !task_cancelled.get() {
                    levels.set(Some(result));
                }
            });
            move || cancelled.set(true)
        });
    }

    let on_preload = {
        let loader = loader.clone();
        let preload = preload.clone();
        let preloading = preloading.clone();
        Callback::from(move |_| {
            if let Ok(loader) = &*loader {
                let loader = loader.clone();
                let preload = preload.clone();
                let preloading = preloading.clone();
                preloading.set(true);
                wasm_bindgen_futures::spawn_local(async move {
                    let result = loader
                        .loader()
                        .preload_game()
                        .await
                        .map(|game| {
                            (
                                game.game_paths.len(),
                                game.challenge_factory.challenge_types.len(),
                            )
                        })
                        .map_err(|error| error.to_string());
                    preload.set(Some(result));
                    preloading.set(false);
                });
            }
        })
    };

    html! {
        <section class="game-loader-preview flex flex-col gap-4 p-4">
            <h2>{i18n.t("Game loader")}</h2>
            {match &*levels {
                None => html! { <p role="status">{i18n.t("Loading...")}</p> },
                Some(Err(error)) => html! { <p role="alert" class="text-error">{error}</p> },
                Some(Ok(levels)) => html! {
                    <>
                        {for levels.iter().map(|level| html! {
                            <div class="flex flex-col gap-2">
                                <h3>{&level.name}</h3>
                                <div class="flex flex-wrap gap-2">
                                    {for level.challenges.iter().map(|config| {
                                        let selected = selected.clone();
                                        let config = config.clone();
                                        let label = config.name.clone();
                                        html! {
                                            <button class="btn btn-primary"
                                                onclick={Callback::from(move |_| selected.set(Some(config.clone())))}>
                                                {label}
                                            </button>
                                        }
                                    })}
                                </div>
                            </div>
                        })}
                        <button class="btn btn-secondary" onclick={on_preload} disabled={*preloading}>
                            {i18n.t("Preload game")}
                        </button>
                    </>
                },
            }}
            if let Some(result) = &*preload {
                <div role="status">
                    {match result {
                        Ok((levels, datasets)) => html! {
                            <p>{format!("{}: {levels}; {}: {datasets}", i18n.t("Levels"), i18n.t("Datasets"))}</p>
                        },
                        Err(error) => html! { <p class="text-error">{error}</p> },
                    }}
                </div>
            }
            if let (Some(config), Ok(loader)) = (&*selected, &*loader) {
                <AssetChallenge key={config.id.clone()}
                    asset_path={preview_game_assets::challenge_path(&config.challenge)}
                    config={config.clone()}
                    loader={loader.clone()}
                    language={Some("de".to_owned())}
                    show_loader_controls={true}
                />
            }
        </section>
    }
}

mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(GameLoaderPreview, (),);
}
