use crate::components::{ChallengeRatingComponent, RatingStarsComponent};
use crate::i18n::use_i18n;
use crate::tools::{update_trace_from_response, TracedRequest};
use gloo::net::http::Request;
use konnektoren_core::challenges::Review;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeReviewProps {
    pub challenge_id: String,
    pub api_url: String,
}

#[function_component(ChallengeReviewComponent)]
pub fn challenge_review(props: &ChallengeReviewProps) -> Html {
    let i18n = use_i18n();
    let stars = use_state(|| 0); // Holds the currently selected star rating
    let comment = use_state(|| String::new()); // Holds the user's comment
    let is_sending = use_state(|| false); // Tracks when the review is being sent
    let is_sent = use_state(|| false); // Tracks whether the review has been successfully sent

    // Handles when a user clicks a star to select a rating
    let on_star_click = {
        let stars = stars.clone();
        Callback::from(move |index: usize| {
            stars.set(index + 1);
        })
    };

    // Handles comment input changes
    let on_comment_change = {
        let comment = comment.clone();
        Callback::from(move |e: InputEvent| {
            let input = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            comment.set(input);
        })
    };

    // Handles the submission of the review
    let on_submit = {
        let is_sending = is_sending.clone();
        let is_sent = is_sent.clone();
        let stars = stars.clone();
        let comment = comment.clone();
        let challenge_id = props.challenge_id.clone();
        let api_url = props.api_url.clone();

        Callback::from(move |_: MouseEvent| {
            let is_sending = is_sending.clone();
            let is_sent = is_sent.clone();

            if *is_sending || *is_sent {
                return; // If it's already sending or sent, do nothing
            }

            is_sending.set(true); // Set the sending in process flag to true

            let stars = *stars;
            let comment = (*comment).clone();
            let challenge_id = challenge_id.clone();
            let api_url = api_url.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let review = Review {
                    challenge_id,
                    rating: stars as u8,
                    comment: if comment.is_empty() {
                        None
                    } else {
                        Some(comment)
                    },
                };

                match Request::post(&api_url)
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
                            is_sent.set(true); // Mark the review as successfully sent
                        } else {
                            log::error!("Failed to submit review: status code {}", status);
                        }
                    }
                    Err(e) => {
                        log::error!("Error while submitting review: {:?}", e);
                    }
                }

                is_sending.set(false); // Release sending lock
            });
        })
    };

    html! {
        <div class="challenge-review">
            <h3>{i18n.t("Rate this Challenge")}</h3>
            <ChallengeRatingComponent api_url={props.api_url.clone()} challenge_id={props.challenge_id.clone()} />
            <RatingStarsComponent max_stars={5} rating={*stars as f64} on_click={Some(on_star_click)} />

            <div class="challenge-review__comment-input">
                <input
                    type="text"
                    placeholder={i18n.t("Leave a comment")}
                    value={(*comment).clone()}
                    oninput={on_comment_change}
                />
            </div>

            // Conditionally hide the button if review is sent; show only if not sent
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
            challenge_id: "123".to_string(),
            api_url: "https://api.example.com/reviews".to_string(),
        },
    );
}
