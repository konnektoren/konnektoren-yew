//! Compose app-specific challenge presentations with the built-in components.
//!
//! An app creates renderers that return `Some(html)` for the challenge and variant they
//! support, then lists its groups before [`core_challenges`]. This keeps app variants of
//! core challenges ahead of the default renderer:
//!
//! ```ignore
//! let music = create_challenge_group!("Music", note_reading(), melody());
//! let groups = create_challenge_groups!(music, core_challenges(App::first_embedding()));
//! let view = render_challenge(&groups, &context).unwrap_or_default();
//! ```
//!
//! Register result presentations in the same group and render them when the host wants to
//! show an answered attempt:
//!
//! ```ignore
//! let music = create_challenge_group!("Music";
//!     challenges [note_reading(), melody()];
//!     results [note_reading_result(), melody_result()]);
//! let groups = create_challenge_groups!(music, core_challenges(App::first_embedding()));
//! let result_view = render_challenge_result(&groups, &context);
//! ```
//!
//! `core_challenges` includes Yew's default challenge and result components. It declines
//! unsupported challenge/result combinations, allowing later groups or the host to decide
//! what to display. A result renderer is only called when the attempt has a result; the host
//! decides when an attempt is finished.
//!
//! Renderers receive the original app attempt and command callback, so each app decides how
//! its components map their props and commands. Groups are UI registration only: challenge
//! families and their serialized representation continue to be defined in core.

use super::{
    ContextualChoiceResultComponent, CustomResultComponent, DialogResultComponent,
    EmbeddedChallengeComponent, GapFillResultComponent, InformativeResultComponent,
    MultipleChoiceResultComponent, OrderingResultComponent,
    sort_table_result::SortTableResultComponent,
};
use konnektoren_core::challenges::{
    Attempt, ChallengeEmbedding, ChallengeResult, ChallengeType, GameChallenge,
};
use konnektoren_core::commands::Command;
use konnektoren_core::events::Event;
use std::rc::Rc;
use yew::prelude::*;

/// Data shared by the renderers in a challenge group.
pub struct ChallengeRenderContext<C: GameChallenge> {
    pub attempt: Attempt<C>,
    pub on_command: Callback<Command<C>>,
    pub on_event: Option<Callback<Event>>,
    pub language: Option<String>,
    pub preview: bool,
}

/// A presentation returns `None` when it does not support an attempt.
type RenderFn<C> = dyn Fn(&ChallengeRenderContext<C>) -> Option<Html>;
pub struct ChallengeRenderer<C: GameChallenge>(Rc<RenderFn<C>>);

impl<C: GameChallenge> Clone for ChallengeRenderer<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<C: GameChallenge> ChallengeRenderer<C> {
    pub fn new(render: impl Fn(&ChallengeRenderContext<C>) -> Option<Html> + 'static) -> Self {
        Self(Rc::new(render))
    }
}

/// Declare a renderer for a challenge and presentation variant.
///
/// The patterns are matched against references to the app's challenge type and its
/// optional variant. The body builds the component with whatever props the host needs.
///
/// ```ignore
/// create_challenge! {
///     pub fn melody(App) {
///         (App::Second(MusicChallenge::Melody(_)), Some(AppVariant::Second(MusicVariant::Melody)))
///             => |context| html! { <MelodyPage attempt={context.attempt.clone()} /> }
///     }
/// }
/// ```
#[macro_export]
macro_rules! create_challenge {
    (
        $vis:vis fn $name:ident($challenge:ty) {
            ($content:pat, $variant:pat) => |$context:ident| $render:expr $(,)?
        }
    ) => {
        $vis fn $name() -> $crate::components::ChallengeRenderer<$challenge> {
            $crate::components::ChallengeRenderer::new(
                move |$context: &$crate::components::ChallengeRenderContext<$challenge>| {
                    match (&$context.attempt.challenge_type, &$context.attempt.challenge_config.variant) {
                        ($content, $variant) => Some($render),
                        _ => None,
                    }
                },
            )
        }
    };
}

/// Declare another presentation of an existing challenge type.
///
/// This uses the same matching rules as [`create_challenge!`], but makes the intent of
/// registrations such as note reading for core's multiple choice explicit.
#[macro_export]
macro_rules! create_challenge_variant {
    ($($declaration:tt)*) => {
        $crate::create_challenge! { $($declaration)* }
    };
}

/// Declare a result presentation. It is considered only when an answer exists.
/// The three patterns match the challenge, presentation variant, and result.
///
/// ```ignore
/// create_challenge_result! {
///     pub fn melody_result(App) {
///         (App::Second(MusicChallenge::Melody(_)), _,
///          ChallengeFamilyResult::Second(MusicResult::Melody(_)))
///             => |context| html! { <MelodyResult attempt={context.attempt.clone()} /> }
///     }
/// }
/// ```
#[macro_export]
macro_rules! create_challenge_result {
    (
        $vis:vis fn $name:ident($challenge:ty) {
            ($content:pat, $variant:pat, $result:pat) => |$context:ident| $render:expr $(,)?
        }
    ) => {
        $vis fn $name() -> $crate::components::ChallengeRenderer<$challenge> {
            $crate::components::ChallengeRenderer::new(
                move |$context: &$crate::components::ChallengeRenderContext<$challenge>| {
                    match (&$context.attempt.challenge_type, &$context.attempt.challenge_config.variant, $context.attempt.challenge_result.as_ref()) {
                        ($content, $variant, Some($result)) => Some($render),
                        _ => None,
                    }
                },
            )
        }
    };
}

/// Declare a result presentation for a particular challenge variant.
#[macro_export]
macro_rules! create_challenge_result_variant {
    ($($declaration:tt)*) => { $crate::create_challenge_result! { $($declaration)* } };
}

/// Presentations in declaration order. Put more specific presentations first.
pub struct ChallengeGroup<C: GameChallenge> {
    pub name: &'static str,
    renderers: Vec<ChallengeRenderer<C>>,
    result_renderers: Vec<ChallengeRenderer<C>>,
}

impl<C: GameChallenge> ChallengeGroup<C> {
    pub fn new(name: &'static str, renderers: Vec<ChallengeRenderer<C>>) -> Self {
        Self {
            name,
            renderers,
            result_renderers: Vec::new(),
        }
    }

    /// Add result presentations in precedence order.
    pub fn with_results(mut self, renderers: Vec<ChallengeRenderer<C>>) -> Self {
        self.result_renderers = renderers;
        self
    }

    /// The first renderer accepting the attempt wins.
    pub fn render(&self, context: &ChallengeRenderContext<C>) -> Option<Html> {
        self.renderers.iter().find_map(|render| (render.0)(context))
    }

    /// Return the first matching result view, if an answer has been recorded.
    pub fn render_result(&self, context: &ChallengeRenderContext<C>) -> Option<Html> {
        context.attempt.challenge_result.as_ref()?;
        self.result_renderers
            .iter()
            .find_map(|render| (render.0)(context))
    }
}

/// Create a named group of typed challenge presentations.
#[macro_export]
macro_rules! create_challenge_group {
    ($name:expr; challenges [$($renderer:expr),* $(,)?]; results [$($result:expr),* $(,)?] $(;)?) => {
        $crate::components::ChallengeGroup::new($name, vec![$($renderer),*]).with_results(vec![$($result),*])
    };
    ($name:expr $(, $renderer:expr)* $(,)?) => {
        $crate::components::ChallengeGroup::new($name, vec![$($renderer),*])
    };
}

/// Combine challenge groups in precedence order.
#[macro_export]
macro_rules! create_challenge_groups {
    ($($group:expr),* $(,)?) => {
        vec![$($group),*]
    };
}

/// Find the first presentation that supports this challenge.
pub fn render_challenge<C: GameChallenge>(
    groups: &[ChallengeGroup<C>],
    context: &ChallengeRenderContext<C>,
) -> Option<Html> {
    groups.iter().find_map(|group| group.render(context))
}

/// Find the first result presentation that supports this answered challenge.
pub fn render_challenge_result<C: GameChallenge>(
    groups: &[ChallengeGroup<C>],
    context: &ChallengeRenderContext<C>,
) -> Option<Html> {
    groups.iter().find_map(|group| group.render_result(context))
}

/// Core's existing UI, for an app challenge type containing core challenges.
pub fn core_challenges<C: GameChallenge>(
    embedding: ChallengeEmbedding<ChallengeType, C>,
) -> ChallengeGroup<C> {
    let result_embedding = embedding.clone();
    ChallengeGroup::new(
        "Core",
        vec![ChallengeRenderer::new(move |context| {
            let inner = embedding.extract_attempt(&context.attempt)?;
            if context.attempt.challenge_config.variant.is_some()
                && inner.challenge_config.variant.is_none()
            {
                return None;
            }
            Some(html! {
                <EmbeddedChallengeComponent<C>
                    attempt={context.attempt.clone()}
                    embedding={embedding.clone()}
                    on_command={context.on_command.clone()}
                    on_event={context.on_event.clone()}
                    language={context.language.clone()}
                    preview={context.preview} />
            })
        })],
    ).with_results(vec![ChallengeRenderer::new(move |context| {
        let inner = result_embedding.extract_attempt(&context.attempt)?;
        let result = inner.challenge_result?;
        let view = match (&inner.challenge_type, &result) {
            (ChallengeType::MultipleChoice(challenge), ChallengeResult::MultipleChoice(_)) => html! {
                <MultipleChoiceResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
            },
            (ChallengeType::ContextualChoice(challenge), ChallengeResult::ContextualChoice(_)) => html! {
                <ContextualChoiceResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
            },
            (ChallengeType::GapFill(challenge), ChallengeResult::GapFill(_)) => html! {
                <GapFillResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
            },
            (ChallengeType::SortTable(challenge), ChallengeResult::SortTable(_)) => html! {
                <SortTableResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
            },
            (ChallengeType::Ordering(challenge), ChallengeResult::Ordering(_)) => html! {
                <OrderingResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
            },
            (ChallengeType::Dialog(challenge), ChallengeResult::Dialog(_)) => html! {
                <DialogResultComponent challenge={challenge.clone()} challenge_result={result.clone()} variant={inner.challenge_config.variant.clone()} />
            },
            (ChallengeType::Custom(challenge), ChallengeResult::Custom(answer)) => html! {
                <CustomResultComponent challenge={challenge.clone()} result={answer.clone()} />
            },
            (ChallengeType::Informative(_), ChallengeResult::Informative) => html! { <InformativeResultComponent /> },
            _ => return None,
        };
        Some(view)
    })])
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use konnektoren_core::challenges::{
        Challenge, ChallengeConfig, ChallengeFamily, ChallengeFamilyResult, ChallengeFamilyVariant,
        ChallengeVariant,
    };

    type App = ChallengeFamily<ChallengeType, ChallengeType>;

    crate::create_challenge_variant! {
        fn note_reading(App) {
            (App::First(ChallengeType::MultipleChoice(_)),
             Some(ChallengeFamilyVariant::Second(ChallengeVariant::DialogQuiz)))
                => |_context| html! { <p class="music-variant">{"music presentation"}</p> }
        }
    }

    crate::create_challenge! {
        fn melody(App) {
            (App::Second(_), _) => |_context| html! { <p class="music-challenge">{"music challenge"}</p> }
        }
    }

    crate::create_challenge_result_variant! {
        fn note_reading_result(App) {
            (App::First(ChallengeType::MultipleChoice(_)),
             Some(ChallengeFamilyVariant::Second(ChallengeVariant::DialogQuiz)),
             ChallengeFamilyResult::First(ChallengeResult::MultipleChoice(_)))
                => |_context| html! { <p class="music-result">{"music result"}</p> }
        }
    }

    crate::create_challenge_result! {
        fn melody_result(App) {
            (App::Second(_), _, ChallengeFamilyResult::Second(ChallengeResult::Informative))
                => |_context| html! { <p class="melody-result">{"melody result"}</p> }
        }
    }

    #[derive(Properties, PartialEq)]
    struct HostProps {
        mode: u8,
    }

    #[function_component(Host)]
    fn host(props: &HostProps) -> Html {
        let (challenge_type, variant) = match props.mode {
            0 => (
                App::First(ChallengeType::default()),
                Some(ChallengeFamilyVariant::Second(ChallengeVariant::DialogQuiz)),
            ),
            1 => (App::First(ChallengeType::default()), None),
            2 => (App::Second(ChallengeType::default()), None),
            _ => (
                App::First(ChallengeType::default()),
                Some(ChallengeFamilyVariant::Second(ChallengeVariant::SortTable)),
            ),
        };
        let context = ChallengeRenderContext {
            attempt: Challenge {
                challenge_type,
                challenge_config: ChallengeConfig::default().with_variant(variant),
                challenge_result: None,
                start_time: None,
                end_time: None,
            },
            on_command: Callback::noop(),
            on_event: None,
            language: None,
            preview: true,
        };
        let music = crate::create_challenge_group!("Music", note_reading(), melody());
        let core = core_challenges(App::first_embedding());
        let groups = crate::create_challenge_groups!(music, core);
        let rendered = render_challenge(&groups, &context);
        html! {
            <crate::i18n::I18nProvider config={crate::i18n::create_i18n_config()}>
                {rendered.unwrap_or_else(|| html! { <p class="unsupported">{"unsupported"}</p> })}
            </crate::i18n::I18nProvider>
        }
    }

    #[tokio::test]
    async fn group_selects_app_variant_app_challenge_and_core_fallback() {
        for (mode, expected) in [
            (0, "music-variant"),
            (1, "class=\"challenge\""),
            (2, "music-challenge"),
            (3, "unsupported"),
        ] {
            let rendered = yew::ServerRenderer::<Host>::with_props(move || HostProps { mode })
                .hydratable(false)
                .render()
                .await;
            assert!(rendered.contains(expected), "{rendered}");
        }
    }

    #[function_component(ResultHost)]
    fn result_host(props: &HostProps) -> Html {
        let (challenge_type, variant, result) = match props.mode {
            0 => (
                App::First(ChallengeType::default()),
                Some(ChallengeFamilyVariant::Second(ChallengeVariant::DialogQuiz)),
                Some(ChallengeFamilyResult::First(
                    ChallengeResult::MultipleChoice(vec![]),
                )),
            ),
            1 => (
                App::First(ChallengeType::default()),
                None,
                Some(ChallengeFamilyResult::First(
                    ChallengeResult::MultipleChoice(vec![]),
                )),
            ),
            2 => (
                App::Second(ChallengeType::default()),
                None,
                Some(ChallengeFamilyResult::Second(ChallengeResult::Informative)),
            ),
            3 => (App::First(ChallengeType::default()), None, None),
            _ => (
                App::First(ChallengeType::default()),
                None,
                Some(ChallengeFamilyResult::First(ChallengeResult::Informative)),
            ),
        };
        let context = ChallengeRenderContext {
            attempt: Challenge {
                challenge_type,
                challenge_config: ChallengeConfig::default().with_variant(variant),
                challenge_result: result,
                start_time: None,
                end_time: None,
            },
            on_command: Callback::noop(),
            on_event: None,
            language: None,
            preview: true,
        };
        let music = crate::create_challenge_group!("Music";
            challenges [note_reading(), melody()];
            results [note_reading_result(), melody_result()]);
        let groups =
            crate::create_challenge_groups!(music, core_challenges(App::first_embedding()));
        let rendered = render_challenge_result(&groups, &context);
        html! {
            <crate::i18n::I18nProvider config={crate::i18n::create_i18n_config()}>
                {rendered.unwrap_or_else(|| html! { <p class="no-result">{"no result"}</p> })}
            </crate::i18n::I18nProvider>
        }
    }

    #[tokio::test]
    async fn groups_render_matching_results_and_reject_absent_or_wrong_results() {
        for (mode, expected) in [
            (0, "music-result"),
            (1, "multiple-choice-result"),
            (2, "melody-result"),
            (3, "no-result"),
            (4, "no-result"),
        ] {
            let rendered =
                yew::ServerRenderer::<ResultHost>::with_props(move || HostProps { mode })
                    .hydratable(false)
                    .render()
                    .await;
            assert!(rendered.contains(expected), "{rendered}");
        }
    }
}
