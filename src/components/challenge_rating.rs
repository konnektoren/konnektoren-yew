use super::review_client::TracedReviewClient;
use crate::i18n::use_i18n;
use konnektoren_yew_review::AverageRating;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeRatingProps {
    pub challenge_id: String,
    pub api_url: String,
    #[prop_or_default]
    pub default_rating: Option<f64>,
    #[prop_or(true)]
    pub default_endpoint: bool,
}

/// Average rating of a challenge, loaded with traced requests.
#[function_component(ChallengeRatingComponent)]
pub fn challenge_rating(props: &ChallengeRatingProps) -> Html {
    let i18n = use_i18n();
    let client = use_memo(
        (props.api_url.clone(), props.default_endpoint),
        |(api_url, default_endpoint)| TracedReviewClient::handle(api_url, *default_endpoint),
    );

    html! {
        <AverageRating
            client={(*client).clone()}
            challenge_id={props.challenge_id.clone()}
            default_rating={props.default_rating}
            loading_label={i18n.t("Loading...")}
        />
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengeRatingComponent,
        ChallengeRatingProps {
            challenge_id: "konnektoren-yew-test".to_string(),
            api_url: "https://api.konnektoren.app/api/v1".to_string(),
            default_rating: Some(4.5),
            default_endpoint: true,
        },
        (
            "loading rating",
            ChallengeRatingProps {
                challenge_id: "konnektoren-yew-test".to_string(),
                api_url: "https://api.konnektoren.app/api/v1".to_string(),
                default_rating: None,
                default_endpoint: true,
            }
        ),
        (
            "with raw url",
            ChallengeRatingProps {
                challenge_id: "konnektoren-yew-test".to_string(),
                api_url: "https://api.konnektoren.app/api/v1/reviews/konnektoren-yew-challenge-rating-test/average".to_string(),
                default_rating: None,
                default_endpoint: false,
            }
        ),
    );
}
