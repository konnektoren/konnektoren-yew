use crate::ReviewClientHandle;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AverageRatingProps {
    pub client: ReviewClientHandle,
    pub challenge_id: String,
    /// Shown until the average is loaded.
    #[prop_or_default]
    pub default_rating: Option<f64>,
    #[prop_or_else(|| "Loading...".to_string())]
    pub loading_label: String,
}

/// The average rating of a challenge, loaded from the review client.
#[function_component(AverageRating)]
pub fn average_rating(props: &AverageRatingProps) -> Html {
    let average = use_state(|| props.default_rating);

    #[cfg(feature = "csr")]
    {
        use crate::ReviewClient;

        let client = props.client.clone();
        let average = average.clone();
        use_effect_with(props.challenge_id.clone(), move |challenge_id| {
            let challenge_id = challenge_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match client.average_rating(challenge_id).await {
                    Ok(value) => average.set(Some(value)),
                    Err(error) => tracing::error!("Failed to fetch the average rating: {error}"),
                }
            });
            || ()
        });
    }

    html! {
        <div class="challenge-rating">
            <span class="challenge-rating__star">{"★"}</span>
            if let Some(average) = *average {
                <span>{format!("{average:.1}")}</span>
            } else {
                <span>{props.loading_label.clone()}</span>
            }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use crate::components::preview_client::PreviewClient;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        AverageRating,
        AverageRatingProps {
            client: PreviewClient::handle(Some(4.5), None),
            challenge_id: "konnektoren-yew-test".to_string(),
            default_rating: Some(4.5),
            loading_label: "Loading...".to_string(),
        },
        (
            "loading rating",
            AverageRatingProps {
                client: PreviewClient::handle(None, None),
                challenge_id: "konnektoren-yew-test".to_string(),
                default_rating: None,
                loading_label: "Loading...".to_string(),
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
    async fn renders_the_default_rating_or_the_loading_label() {
        let client = ReviewClientHandle::new(NoClient);
        let html = yew::LocalServerRenderer::<AverageRating>::with_props(AverageRatingProps {
            client: client.clone(),
            challenge_id: "id".into(),
            default_rating: Some(4.25),
            loading_label: "Loading...".into(),
        })
        .render()
        .await;
        assert!(html.contains("4.2") || html.contains("4.3"));

        let html = yew::LocalServerRenderer::<AverageRating>::with_props(AverageRatingProps {
            client,
            challenge_id: "id".into(),
            default_rating: None,
            loading_label: "Lädt…".into(),
        })
        .render()
        .await;
        assert!(html.contains("Lädt…"));
    }
}
