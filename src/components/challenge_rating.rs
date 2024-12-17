use gloo::net::http::Request;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeRatingProps {
    pub challenge_id: String,
    pub api_url: String,
}

#[function_component(ChallengeRatingComponent)]
pub fn challenge_rating(props: &ChallengeRatingProps) -> Html {
    let average = use_state(|| None as Option<f64>);
    {
        let challenge_id = props.challenge_id.clone();
        let api_url = props.api_url.clone();
        let average = average.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("{}/reviews/{}/average", api_url, challenge_id);
                let response = Request::get(&url).send().await;

                match response {
                    Ok(response) if response.status() == 200 => {
                        if let Ok(avg) = response.json::<f64>().await {
                            average.set(Some(avg));
                        }
                    }
                    _ => {
                        log::error!("Failed to fetch the average rating.");
                    }
                }
            });

            || ()
        });
    }

    html! {
        <div class="challenge-rating">
            <span class="star-symbol">{"★"}</span>
            if let Some(avg) = *average {
                <span>{format!("{:.1}", avg)}</span>
            } else {
                <span>{"Loading..."}</span>
            }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengeRatingComponent,
        ChallengeRatingProps {
            challenge_id: "123".to_string(),
            api_url: "https://api.example.com/reviews".to_string(),
        },
    );
}
