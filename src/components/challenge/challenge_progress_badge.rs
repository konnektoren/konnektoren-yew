use konnektoren_core::challenges::{
    ChallengeConfig, ChallengeHistory, ChallengeResult, Performance,
};
use yew::prelude::*;

/// Lowest performance (in percent) that is shown by the badge.
pub const MIN_VISIBLE_PERCENT: u32 = 50;

/// Best performance (in percent) of all attempts of `config` found in `history`.
///
/// Attempts that were not finished (fewer results than tasks) are ignored. Custom results carry
/// their own performance and always count as finished.
pub fn best_performance(config: &ChallengeConfig, history: &ChallengeHistory) -> Option<u32> {
    history
        .challenges
        .iter()
        .filter(|c| {
            c.challenge_config.id == config.id
                && (matches!(c.challenge_result, ChallengeResult::Custom(_))
                    || c.challenge_result.len() == c.challenge_config.tasks.len())
        })
        .map(|c| c.performance(&c.challenge_result).min(100))
        .max()
}

/// Hue (0 = red, 120 = green) for a percentage between 50 and 100.
pub fn progress_hue(percent: u32) -> u32 {
    let clamped = percent.clamp(MIN_VISIBLE_PERCENT, 100);
    (clamped - MIN_VISIBLE_PERCENT) * 120 / (100 - MIN_VISIBLE_PERCENT)
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct ChallengeProgressBadgeProps {
    /// Best performance in percent.
    pub percent: u32,
    /// Optional accessible label, defaults to the percentage.
    #[prop_or_default]
    pub label: Option<AttrValue>,
}

/// Colored badge for the best performance of a challenge.
///
/// Hidden below 50%, color runs from red (50%) to green (99%), solid green at 100%.
#[function_component(ChallengeProgressBadge)]
pub fn challenge_progress_badge(props: &ChallengeProgressBadgeProps) -> Html {
    if props.percent < MIN_VISIBLE_PERCENT {
        return html! {};
    }
    let percent = props.percent.min(100);
    let complete = percent == 100;
    let class = classes!(
        "challenge-progress-badge",
        complete.then_some("challenge-progress-badge--complete")
    );
    let style = format!("--challenge-progress-hue: {}", progress_hue(percent));
    let label = props
        .label
        .clone()
        .unwrap_or_else(|| AttrValue::from(format!("{percent}%")));

    html! {
        <span class={class} style={style} role="img" aria-label={label}>
            if complete {
                <i class="fas fa-check-circle challenge-progress-badge__icon" aria-hidden="true"></i>
            }
            <span class="challenge-progress-badge__value" aria-hidden="true">{format!("{percent}%")}</span>
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hue_runs_red_to_green() {
        assert_eq!(progress_hue(50), 0);
        assert_eq!(progress_hue(75), 60);
        assert_eq!(progress_hue(100), 120);
        assert_eq!(progress_hue(10), 0);
        assert_eq!(progress_hue(150), 120);
    }

    #[test]
    fn no_attempt_has_no_performance() {
        let config = ChallengeConfig::default();
        assert_eq!(best_performance(&config, &ChallengeHistory::new()), None);
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengeProgressBadge,
        ChallengeProgressBadgeProps {
            percent: 50,
            label: None
        },
        (
            "75%",
            ChallengeProgressBadgeProps {
                percent: 75,
                label: None
            }
        ),
        (
            "90%",
            ChallengeProgressBadgeProps {
                percent: 90,
                label: None
            }
        ),
        (
            "100%",
            ChallengeProgressBadgeProps {
                percent: 100,
                label: None
            }
        ),
        (
            "Below 50% (hidden)",
            ChallengeProgressBadgeProps {
                percent: 30,
                label: None
            }
        )
    );
}
