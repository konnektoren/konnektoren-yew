use super::review_client::TracedReviewClient;
use crate::i18n::use_i18n;
use konnektoren_yew_review::{ReviewForm, ReviewLabels};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeReviewProps {
    pub challenge_id: String,
    pub api_url: String,
    #[prop_or(true)]
    pub default_endpoint: bool,
}

/// Review form of the `konnektoren-yew-review` crate, translated and sent with traced requests.
///
/// With the `web-analytics` feature, submitted and failed reviews are tracked.
#[function_component(ChallengeReviewComponent)]
pub fn challenge_review(props: &ChallengeReviewProps) -> Html {
    let i18n = use_i18n();
    let client = use_memo(
        (props.api_url.clone(), props.default_endpoint),
        |(api_url, default_endpoint)| TracedReviewClient::handle(api_url, *default_endpoint),
    );
    let labels = ReviewLabels {
        title: i18n.t("Rate this Challenge"),
        comment_placeholder: i18n.t("Leave a comment"),
        submit: i18n.t("Submit"),
        submitting: i18n.t("Submitting..."),
        thank_you: i18n.t("Thank you for your review!"),
        loading: i18n.t("Loading..."),
    };

    html! {
        <ReviewForm client={(*client).clone()} challenge_id={props.challenge_id.clone()} {labels} />
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengeReviewComponent,
        ChallengeReviewProps {
            challenge_id: "konnektoren-yew-test".to_string(),
            api_url: "https://api.konnektoren.app/api/v1".to_string(),
            default_endpoint: true,
        },
        (
            "with raw url",
            ChallengeReviewProps {
                challenge_id: "konnektoren-yew-test".to_string(),
                api_url: "https://api.konnektoren.app/reviews".to_string(),
                default_endpoint: false,
            }
        ),
        (
            "error (wrong url)",
            ChallengeReviewProps {
                challenge_id: "konnektoren-yew-test".to_string(),
                api_url: "https://invalid.example.invalid/api/v1".to_string(),
                default_endpoint: true,
            }
        ),
    );
}
