use super::ChallengeComponent;
use crate::providers::embed_command_callback;
use konnektoren_core::challenges::{Attempt, ChallengeEmbedding, ChallengeType, GameChallenge};
use konnektoren_core::commands::Command;
use konnektoren_core::events::Event;
use yew::prelude::*;

#[derive(Properties)]
pub struct EmbeddedChallengeProps<Outer: GameChallenge> {
    /// The application's active attempt, of its own challenge type.
    pub attempt: Attempt<Outer>,
    /// How core's built-in challenges are included in the application's type.
    pub embedding: ChallengeEmbedding<ChallengeType, Outer>,
    /// Receives the commands of the default components, converted to the application's.
    pub on_command: Callback<Command<Outer>>,
    #[prop_or_default]
    pub on_event: Option<Callback<Event>>,
    #[prop_or_default]
    pub language: Option<String>,
    #[prop_or(false)]
    pub preview: bool,
    /// Shown when the attempt is not of a built-in type; render the application's own UI here.
    #[prop_or_default]
    pub fallback: Html,
}

impl<Outer: GameChallenge> PartialEq for EmbeddedChallengeProps<Outer> {
    fn eq(&self, other: &Self) -> bool {
        self.attempt == other.attempt
            && self.embedding == other.embedding
            && self.on_command == other.on_command
            && self.on_event == other.on_event
            && self.language == other.language
            && self.preview == other.preview
            && self.fallback == other.fallback
    }
}

/// Renders an attempt of an application's extended challenge type with the default
/// component of the built-in challenge it holds, or the `fallback` for the application's own.
#[function_component(EmbeddedChallengeComponent)]
pub fn embedded_challenge_component<Outer: GameChallenge>(
    props: &EmbeddedChallengeProps<Outer>,
) -> Html {
    let Some(challenge) = props.embedding.extract_attempt(&props.attempt) else {
        return props.fallback.clone();
    };
    let on_command = Some(embed_command_callback(
        props.on_command.clone(),
        props.embedding.clone(),
    ));
    let variant = challenge.challenge_config.variant.clone();
    html! {
        <ChallengeComponent {challenge} {variant} {on_command}
            on_event={props.on_event.clone()} language={props.language.clone()}
            preview={props.preview} />
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use konnektoren_core::challenges::{Challenge, ChallengeConfig, ChallengeFamily};

    type Extended = ChallengeFamily<ChallengeType, ChallengeType>;

    fn props(challenge_type: Extended) -> EmbeddedChallengeProps<Extended> {
        EmbeddedChallengeProps {
            attempt: Challenge {
                challenge_type,
                challenge_config: ChallengeConfig::default().with_variant(None),
                challenge_result: None,
                start_time: None,
                end_time: None,
            },
            embedding: Extended::first_embedding(),
            on_command: Callback::noop(),
            on_event: None,
            language: None,
            preview: true,
            fallback: html! { <p class="application-own">{"application UI"}</p> },
        }
    }

    #[derive(Properties, PartialEq)]
    struct HostProps {
        built_in: bool,
    }

    /// The component inside the providers the default components expect.
    #[function_component(Host)]
    fn host(host: &HostProps) -> Html {
        let challenge_type = if host.built_in {
            ChallengeFamily::First(ChallengeType::default())
        } else {
            ChallengeFamily::Second(ChallengeType::default())
        };
        html! {
            <crate::i18n::I18nProvider config={crate::i18n::create_i18n_config()}>
                <EmbeddedChallengeComponent<Extended> ..props(challenge_type) />
            </crate::i18n::I18nProvider>
        }
    }

    async fn render(built_in: bool) -> String {
        yew::ServerRenderer::<Host>::with_props(move || HostProps { built_in })
            .hydratable(false)
            .render()
            .await
    }

    #[tokio::test]
    async fn built_in_branch_uses_the_default_component() {
        let html = render(true).await;
        assert!(html.contains("class=\"challenge\""), "{html}");
        assert!(!html.contains("application-own"));
    }

    #[tokio::test]
    async fn application_branch_renders_the_fallback() {
        let html = render(false).await;
        assert!(html.contains("application-own"), "{html}");
        assert!(!html.contains("class=\"challenge\""));
    }
}
