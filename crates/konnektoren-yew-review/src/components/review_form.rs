use crate::{AverageRating, RatingStarsComponent, ReviewClientHandle, ReviewLabels};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ReviewFormProps {
    pub client: ReviewClientHandle,
    pub challenge_id: String,
    #[prop_or_default]
    pub labels: ReviewLabels,
}

/// Star rating with an optional comment, sent through the review client.
#[function_component(ReviewForm)]
pub fn review_form(props: &ReviewFormProps) -> Html {
    let stars = use_state(|| 0_usize);
    let comment = use_state(String::new);
    let is_sending = use_state(|| false);
    let is_sent = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    #[cfg(all(feature = "web-analytics", feature = "csr"))]
    let analytics = konnektoren_yew_web_analytics::use_web_analytics();

    let on_star_click = {
        let stars = stars.clone();
        Callback::from(move |index: usize| stars.set(index + 1))
    };

    #[cfg(feature = "csr")]
    let on_comment_change = {
        let comment = comment.clone();
        Callback::from(move |event: InputEvent| {
            comment.set(
                event
                    .target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
        })
    };
    #[cfg(not(feature = "csr"))]
    let on_comment_change = Callback::noop();

    #[cfg(feature = "csr")]
    let on_submit = {
        use crate::{Review, ReviewClient};

        let (stars, comment) = (stars.clone(), comment.clone());
        let (is_sending, is_sent, error) = (is_sending.clone(), is_sent.clone(), error.clone());
        let client = props.client.clone();
        let challenge_id = props.challenge_id.clone();
        #[cfg(feature = "web-analytics")]
        let analytics = analytics.clone();

        Callback::from(move |_: MouseEvent| {
            if *is_sending || *is_sent {
                return;
            }
            is_sending.set(true);
            error.set(None);

            let rating = u8::try_from(*stars).unwrap_or(u8::MAX);
            let comment = (*comment).clone();
            let review = Review {
                challenge_id: challenge_id.clone(),
                rating,
                comment: (!comment.is_empty()).then_some(comment),
            };
            let (is_sending, is_sent, error) = (is_sending.clone(), is_sent.clone(), error.clone());
            let client = client.clone();
            #[cfg(feature = "web-analytics")]
            let analytics = analytics.clone();

            wasm_bindgen_futures::spawn_local(async move {
                #[cfg(feature = "web-analytics")]
                let parameters = crate::review_parameters(
                    &review.challenge_id,
                    review.rating,
                    review.comment.is_some(),
                );
                let result = client.submit(review).await;
                #[cfg(feature = "web-analytics")]
                if let Some(analytics) = &analytics {
                    use konnektoren_yew_web_analytics::WebAnalytics;
                    let mut parameters = parameters;
                    let event = match &result {
                        Ok(()) => crate::REVIEW_SUBMITTED_EVENT,
                        Err(reason) => {
                            parameters.insert("reason".into(), reason.reason().into());
                            crate::REVIEW_FAILED_EVENT
                        }
                    };
                    let _ = analytics.track_event(event, parameters);
                }
                match result {
                    Ok(()) => {
                        tracing::info!("Review submitted successfully");
                        is_sent.set(true);
                    }
                    Err(reason) => {
                        tracing::error!("Failed to submit review: {reason}");
                        error.set(Some(reason.to_string()));
                    }
                }
                is_sending.set(false);
            });
        })
    };
    #[cfg(not(feature = "csr"))]
    let on_submit = Callback::noop();

    let labels = &props.labels;
    html! {
        <div class="challenge-review">
            <h3>{labels.title.clone()}</h3>
            <AverageRating
                client={props.client.clone()}
                challenge_id={props.challenge_id.clone()}
                loading_label={labels.loading.clone()}
            />
            <RatingStarsComponent
                max_stars={5}
                rating={*stars as f64}
                on_click={Some(on_star_click)}
            />

            <div class="challenge-review__comment-input">
                <input
                    type="text"
                    placeholder={labels.comment_placeholder.clone()}
                    value={(*comment).clone()}
                    oninput={on_comment_change}
                />
            </div>

            if let Some(error) = (*error).clone() {
                <p class="challenge-review__error">{error}</p>
            }

            if !*is_sent {
                <button
                    onclick={on_submit}
                    disabled={*stars == 0 || *is_sending}
                >
                    {if *is_sending { labels.submitting.clone() } else { labels.submit.clone() }}
                </button>
            } else {
                <p class="challenge-review__success">{labels.thank_you.clone()}</p>
            }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use crate::ReviewError;
    use crate::components::preview_client::PreviewClient;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ReviewForm,
        ReviewFormProps {
            client: PreviewClient::handle(Some(4.2), None),
            challenge_id: "konnektoren-yew-test".to_string(),
            labels: ReviewLabels::default(),
        },
        (
            "submit fails",
            ReviewFormProps {
                client: PreviewClient::handle(Some(3.0), Some(ReviewError::Status(500))),
                challenge_id: "konnektoren-yew-test".to_string(),
                labels: ReviewLabels::default(),
            }
        ),
        (
            "server unreachable",
            ReviewFormProps {
                client: PreviewClient::handle(None, Some(ReviewError::Network)),
                challenge_id: "konnektoren-yew-test".to_string(),
                labels: ReviewLabels::default(),
            }
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Review, ReviewClient, ReviewError, ReviewFuture};

    struct NoClient;

    impl ReviewClient for NoClient {
        fn submit(&self, _: Review) -> ReviewFuture<()> {
            Box::pin(std::future::ready(Err(ReviewError::Unavailable)))
        }
        fn average_rating(&self, _: String) -> ReviewFuture<f64> {
            Box::pin(std::future::ready(Err(ReviewError::Unavailable)))
        }
    }

    #[tokio::test]
    async fn renders_the_translated_labels() {
        let labels = ReviewLabels::translated(|key| format!("de:{key}"));
        let html = yew::LocalServerRenderer::<ReviewForm>::with_props(ReviewFormProps {
            client: ReviewClientHandle::new(NoClient),
            challenge_id: "id".into(),
            labels,
        })
        .render()
        .await;
        assert!(html.contains("de:Rate this Challenge"));
        assert!(html.contains("de:Leave a comment"));
        assert!(html.contains("de:Submit"));
        assert!(html.contains("disabled"));
    }
}
