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

#[function_component(ChallengeReviewComponent)]
pub fn challenge_review(props: &ChallengeReviewProps) -> Html {
    let i18n = use_i18n();
    let stars = use_state(|| 0);
    let comment = use_state(String::new);
    let is_sending = use_state(|| false);
    let is_sent = use_state(|| false);

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
        let stars = stars.clone();
        let comment = comment.clone();
        let challenge_id = props.challenge_id.clone();
        let api_url = props.api_url.clone();
        let default_endpoint = props.default_endpoint;

        Callback::from(move |_: MouseEvent| {
            #[cfg(feature = "csr")]
            {
                use crate::tools::{TracedRequest, update_trace_from_response};
                use gloo::net::http::Request;
                use konnektoren_core::challenges::Review;

                let is_sending = is_sending.clone();
                let is_sent = is_sent.clone();

                if *is_sending || *is_sent {
                    return;
                }

                is_sending.set(true);

                let stars = *stars;
                let comment = (*comment).clone();
                let challenge_id = challenge_id.clone();
                let api_url = api_url.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let review = Review {
                        challenge_id: challenge_id.clone(),
                        rating: stars as u8,
                        comment: if comment.is_empty() {
                            None
                        } else {
                            Some(comment)
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
                                log::info!("Review submitted successfully");
                                is_sent.set(true);
                            } else {
                                log::error!("Failed to submit review: status code {}", status);
                            }
                        }
                        Err(e) => {
                            log::error!("Error while submitting review: {:?}", e);
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

            if !*is_sent {
                <button
                    onclick={on_submit}
                    disabled={*stars == 0 || *is_sending}
                >
                    {if *is_sending { i18n.t("Submitting...") } else { i18n.t("Submit") }}
                </button>
            } else {
                <p>{i18n.t("Thank you for your review!")}</p>
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
    );
}
