use gloo::net::http::Request;
use yew::prelude::*;

use crate::tools::TracedResponse;

#[derive(Properties, PartialEq)]
pub struct ChallengePresenceProps {
    pub challenge_id: String,
    pub api_url: String,
    #[prop_or(false)]
    pub read_only: bool,
}

#[function_component(ChallengePresenceComponent)]
pub fn challenge_presence(props: &ChallengePresenceProps) -> Html {
    let count = use_state(|| None as Option<u32>);

    {
        let challenge_id = props.challenge_id.clone();
        let api_url = props.api_url.clone();
        let count = count.clone();
        let read_only = props.read_only;

        use_effect_with(challenge_id.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Choose endpoint based on read_only prop
                let url = if read_only {
                    format!("{}/challenges/{}/presence", api_url, challenge_id)
                } else {
                    format!("{}/challenges/{}/presence/record", api_url, challenge_id)
                };

                // Use GET for read-only, POST for recording presence
                let response = if read_only {
                    Request::get(&url)
                } else {
                    Request::post(&url)
                }
                .send_traced()
                .await;

                match response {
                    Ok(response) if response.status() == 200 => {
                        if let Ok(stats) = response.json::<ChallengePresenceStats>().await {
                            count.set(Some(stats.count));
                        }
                    }
                    _ => {
                        log::error!("Failed to fetch presence count.");
                    }
                }
            });

            || ()
        });
    }

    html! {
        <div class="presence-badge">
            <i class="fas fa-users"></i>
            if let Some(num) = *count {
                <span>{num}</span>
            } else {
                <span>{"..."}</span>
            }
        </div>
    }
}
#[derive(serde::Deserialize)]
struct ChallengePresenceStats {
    count: u32,
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengePresenceComponent,
        ChallengePresenceProps {
            challenge_id: "123".to_string(),
            api_url: "https://api.example.com".to_string(),
            read_only: true,
        },
    );
}
