use crate::components::{ChallengeRatingComponent, RatingStarsComponent};
use crate::i18n::use_i18n;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeReviewProps {
    pub challenge_id: String,
    pub api_url: String,
    #[prop_or(true)]
    pub default_endpoint: bool,
}

fn build_review_url(api_url: &str, default_endpoint: bool) -> String {
    if !default_endpoint {
        return api_url.to_string();
    }

    let base = api_url.trim_end_matches('/');
    format!("{}/reviews", base)
}

/// Analytics event sent after the API accepted a review.
#[cfg(feature = "web-analytics")]
pub const REVIEW_SUBMITTED_EVENT: &str = "challenge_review_submitted";
/// Analytics event sent when a review could not be submitted.
#[cfg(feature = "web-analytics")]
pub const REVIEW_FAILED_EVENT: &str = "challenge_review_failed";

/// Event parameters for a review. The comment text is never sent, only whether one was given.
#[cfg(feature = "web-analytics")]
#[cfg_attr(not(feature = "csr"), allow(dead_code))]
fn review_parameters(
    challenge_id: &str,
    rating: u8,
    has_comment: bool,
) -> crate::web_analytics::Parameters {
    let mut parameters = crate::web_analytics::Parameters::new();
    parameters.insert("challenge_id".into(), challenge_id.into());
    parameters.insert("rating".into(), rating.into());
    parameters.insert("has_comment".into(), i32::from(has_comment).into());
    parameters
}

#[function_component(ChallengeReviewComponent)]
pub fn challenge_review(props: &ChallengeReviewProps) -> Html {
    let i18n = use_i18n();
    let stars = use_state(|| 0);
    let comment = use_state(String::new);
    let is_sending = use_state(|| false);
    let is_sent = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    #[cfg(all(feature = "web-analytics", feature = "csr"))]
    let analytics = crate::web_analytics::use_web_analytics();

    let on_star_click = {
        let stars = stars.clone();
        Callback::from(move |index: usize| {
            stars.set(index + 1);
        })
    };

    let on_comment_change = {
        let comment = comment.clone();
        Callback::from(move |e: InputEvent| {
            #[cfg(feature = "csr")]
            {
                let input = e
                    .target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value();
                comment.set(input);
            }
        })
    };

    let on_submit = {
        let is_sending = is_sending.clone();
        let is_sent = is_sent.clone();
        let error = error.clone();
        let stars = stars.clone();
        let comment = comment.clone();
        let challenge_id = props.challenge_id.clone();
        let api_url = props.api_url.clone();
        let default_endpoint = props.default_endpoint;
        #[cfg(all(feature = "web-analytics", feature = "csr"))]
        let analytics = analytics.clone();

        Callback::from(move |_: MouseEvent| {
            #[cfg(feature = "csr")]
            {
                use crate::tools::{TracedRequest, update_trace_from_response};
                use gloo::net::http::Request;
                use konnektoren_core::challenges::Review;

                let is_sending = is_sending.clone();
                let is_sent = is_sent.clone();
                let error = error.clone();

                if *is_sending || *is_sent {
                    return;
                }

                is_sending.set(true);
                error.set(None);

                let stars = *stars;
                let comment = (*comment).clone();
                let challenge_id = challenge_id.clone();
                let api_url = api_url.clone();
                #[cfg(feature = "web-analytics")]
                let analytics = analytics.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    #[cfg(feature = "web-analytics")]
                    let track = |event: &str, reason: Option<String>| {
                        use crate::web_analytics::WebAnalytics;
                        if let Some(analytics) = &analytics {
                            let mut parameters =
                                review_parameters(&challenge_id, stars as u8, !comment.is_empty());
                            if let Some(reason) = reason {
                                parameters.insert("reason".into(), reason.into());
                            }
                            let _ = analytics.track_event(event, parameters);
                        }
                    };

                    let review = Review {
                        challenge_id: challenge_id.clone(),
                        rating: stars as u8,
                        comment: if comment.is_empty() {
                            None
                        } else {
                            Some(comment.clone())
                        },
                    };

                    let url = build_review_url(&api_url, default_endpoint);

                    match Request::post(&url)
                        .with_trace()
                        .json(&review)
                        .unwrap()
                        .send()
                        .await
                    {
                        Ok(response) => {
                            update_trace_from_response(&response);
                            let status = response.status();
                            if (200..300).contains(&status) {
                                tracing::info!("Review submitted successfully");
                                is_sent.set(true);
                                #[cfg(feature = "web-analytics")]
                                track(REVIEW_SUBMITTED_EVENT, None);
                            } else {
                                tracing::error!("Failed to submit review: status code {}", status);
                                error.set(Some(format!("Failed to submit ({})", status)));
                                #[cfg(feature = "web-analytics")]
                                track(REVIEW_FAILED_EVENT, Some(format!("status_{status}")));
                            }
                        }
                        Err(e) => {
                            tracing::error!("Error while submitting review: {:?}", e);
                            error.set(Some("Could not reach server".to_string()));
                            #[cfg(feature = "web-analytics")]
                            track(REVIEW_FAILED_EVENT, Some("network".to_string()));
                        }
                    }

                    is_sending.set(false);
                });
            }
        })
    };

    html! {
        <div class="challenge-review">
            <h3>{i18n.t("Rate this Challenge")}</h3>
            <ChallengeRatingComponent
                api_url={props.api_url.clone()}
                challenge_id={props.challenge_id.clone()}
                default_endpoint={props.default_endpoint}
            />
            <RatingStarsComponent
                max_stars={5}
                rating={*stars as f64}
                on_click={Some(on_star_click)}
            />

            <div class="challenge-review__comment-input">
                <input
                    type="text"
                    placeholder={i18n.t("Leave a comment")}
                    value={(*comment).clone()}
                    oninput={on_comment_change}
                />
            </div>

            if let Some(err) = (*error).clone() {
                <p class="challenge-review__error">{err}</p>
            }

            if !*is_sent {
                <button
                    onclick={on_submit}
                    disabled={*stars == 0 || *is_sending}
                >
                    {if *is_sending { i18n.t("Submitting...") } else { i18n.t("Submit") }}
                </button>
            } else {
                <p class="challenge-review__success">{i18n.t("Thank you for your review!")}</p>
            }
        </div>
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

#[cfg(all(test, feature = "web-analytics"))]
mod tests {
    use super::*;

    #[test]
    fn review_parameters_never_contain_the_comment_text() {
        let parameters = review_parameters("music:c-major", 4, true);
        assert_eq!(parameters["challenge_id"], "music:c-major");
        assert_eq!(parameters["rating"], 4);
        assert_eq!(parameters["has_comment"], 1);
        assert_eq!(parameters.len(), 3);
    }

    #[test]
    fn review_parameters_mark_missing_comments() {
        assert_eq!(review_parameters("id", 1, false)["has_comment"], 0);
    }
}
