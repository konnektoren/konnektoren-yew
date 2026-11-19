use yew::prelude::*;

/// One numeric comparison in a core achievement condition.
#[derive(Clone, Debug, PartialEq)]
pub struct CoreAchievementTarget {
    pub statistic: String,
    pub current: f64,
    pub target: f64,
}

/// Read simple `>=` comparisons joined by `&`, using core's own statistics.
/// Returns `None` for other expressions so a caller can provide its own presentation.
pub fn core_achievement_targets(
    condition: &str,
    game: &konnektoren_core::game::Game,
) -> Option<Vec<CoreAchievementTarget>> {
    use konnektoren_core::achievements::{
        AveragePerformance, CompletedGamePaths, DifferentChallengeTypesCompleted, GameStatistics,
        PerfectChallenges, TotalChallenges, TotalXp,
    };
    let stats = GameStatistics::new(game);
    condition
        .split('&')
        .map(|part| {
            let (name, target) = part.trim().split_once(">=")?;
            let name = name.trim();
            let current = match name {
                "total_challenges" => stats.total_challenges() as f64,
                "average_performance" => stats.average_performance(),
                "total_xp" => stats.total_xp() as f64,
                "completed_game_paths" => stats.completed_game_paths() as f64,
                "perfect_challenges" => stats.perfect_challenges() as f64,
                "different_challenge_types_completed" => {
                    stats.different_challenge_types_completed() as f64
                }
                _ => return None,
            };
            Some(CoreAchievementTarget {
                statistic: name.to_owned(),
                current,
                target: target.trim().parse().ok()?,
            })
        })
        .collect()
}

/// A requirement supplied by the host application, with its own labels and links.
#[derive(Clone, PartialEq)]
pub enum AchievementRequirement {
    /// A requirement such as an exercise or a choice between exercises.
    Item { content: Html, completed: bool },
    /// A measured value and its target, formatted for the host's locale.
    Progress { label: String, value: String },
}

#[derive(Properties, PartialEq)]
pub struct AchievementRequirementsProps {
    pub heading: String,
    pub completed_label: String,
    pub incomplete_label: String,
    pub requirements: Vec<AchievementRequirement>,
}

/// Displays requirements without assuming a particular game, DSL or router.
#[function_component(AchievementRequirements)]
pub fn achievement_requirements(props: &AchievementRequirementsProps) -> Html {
    if props.requirements.is_empty() {
        return html! {};
    }
    html! {
        <div class="achievement-requirements">
            <span class="achievement-requirements__heading">{&props.heading}</span>
            <ul class="achievement-requirements__list">
                {for props.requirements.iter().map(|requirement| match requirement {
                    AchievementRequirement::Item { content, completed } => html! {
                        <li class="achievement-requirements__item">
                            <i class={classes!("fa-solid", if *completed { "fa-circle-check" } else { "fa-circle" })} aria-hidden="true"></i>
                            <span><span class="sr-only">{if *completed { &props.completed_label } else { &props.incomplete_label }}{": "}</span>{content.clone()}</span>
                        </li>
                    },
                    AchievementRequirement::Progress { label, value } => html! {
                        <li class="achievement-requirements__metric"><span>{label}</span><strong>{value}</strong></li>
                    },
                })}
            </ul>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::core_achievement_targets;
    use konnektoren_core::game::Game;

    #[test]
    fn parses_core_targets_and_rejects_unsupported_expressions() {
        let game = Game::default();
        let targets =
            core_achievement_targets("total_challenges >= 10 & average_performance >= 80", &game);
        assert!(targets.as_ref().is_some_and(|targets| targets.len() == 2
            && targets[0].current == 0.0
            && targets[0].target == 10.0
            && targets[1].current == 0.0
            && targets[1].target == 80.0));
        assert!(
            core_achievement_targets("total_challenges >= 1 | total_xp >= 100", &game).is_none()
        );
        assert!(core_achievement_targets("completed_exercises: exercise-1", &game).is_none());
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        AchievementRequirements,
        AchievementRequirementsProps {
            heading: "Requirements".into(),
            completed_label: "Completed".into(),
            incomplete_label: "Not completed".into(),
            requirements: vec![
                AchievementRequirement::Progress {
                    label: "Exercises completed".into(),
                    value: "3 / 10".into()
                },
                AchievementRequirement::Item {
                    content: html! { <a href="#exercise">{"First exercise"}</a> },
                    completed: false
                },
            ],
        },
    );
}
