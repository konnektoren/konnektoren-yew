use crate::tools::TracedResponse;
use gloo::net::http::Request;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengePresenceProps {
    pub challenge_id: String,
    pub api_url: String,
    #[prop_or(false)]
    pub read_only: bool,
    #[prop_or(true)]
    pub default_endpoint: bool,
}

#[derive(serde::Deserialize)]
struct ChallengePresenceStats {
    count: u32,
}

fn build_presence_url(
    api_url: &str,
    challenge_id: &str,
    read_only: bool,
    default_endpoint: bool,
) -> String {
    if !default_endpoint {
        return api_url.to_string();
    }

    let base = api_url.trim_end_matches('/');
    if read_only {
        format!("{}/challenges/{}/presence", base, challenge_id)
    } else {
        format!("{}/challenges/{}/presence/record", base, challenge_id)
    }
}

#[function_component(ChallengePresenceComponent)]
pub fn challenge_presence(props: &ChallengePresenceProps) -> Html {
    let count = use_state(|| None as Option<u32>);

    {
        let challenge_id = props.challenge_id.clone();
        let api_url = props.api_url.clone();
        let count = count.clone();
        let read_only = props.read_only;
        let default_endpoint = props.default_endpoint;

        use_effect_with(challenge_id.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let url = build_presence_url(&api_url, &challenge_id, read_only, default_endpoint);

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

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengePresenceComponent,
        ChallengePresenceProps {
            challenge_id: "konnektoren-yew-test".to_string(),
            api_url: "https://api.konnektoren.app/api/v1".to_string(),
            read_only: false,
            default_endpoint: true,
        },
        (
            "readonly",
            ChallengePresenceProps {
                challenge_id: "konnektoren-yew-test".to_string(),
                api_url: "https://api.konnektoren.app/api/v1".to_string(),
                read_only: true,
                default_endpoint: true,
            }
        ),
        (
            "with raw url",
            ChallengePresenceProps {
                challenge_id: "konnektoren-yew-test".to_string(),
                api_url: "https://api.konnektoren.app/api/v1/challenges/raw/presence".to_string(),
                read_only: true,
                default_endpoint: false,
            }
        ),
    );
}
