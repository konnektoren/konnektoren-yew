use crate::components::BuyMeCoffeeComponent;
use rand::Rng;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AdNetwork {
    GoogleAdsense {
        client: String,
        slot: String,
        width: String,
        height: String,
    },
    BuyMeACoffee {
        data_id: String,
        text: &'static str,
        button_colour: &'static str,
    },
}

#[derive(Properties, PartialEq)]
pub struct AdvertisementProps {
    #[prop_or(30)]
    pub show_probability: u8,
    #[prop_or_default]
    pub placement: Option<String>,
    pub networks: Vec<AdNetwork>,
}

#[function_component(AdvertisementComponent)]
pub fn advertisement(props: &AdvertisementProps) -> Html {
    let should_show = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..100) < props.show_probability
    };

    let selected_network = {
        if !should_show || props.networks.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..props.networks.len());
            Some(props.networks[index].clone())
        }
    };

    let ad_blocked = use_state(|| false);

    // Check if ads are blocked
    {
        let ad_blocked = ad_blocked.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Check after a short delay if ads are loaded
                gloo::timers::future::TimeoutFuture::new(2000).await;
                if let Some(window) = web_sys::window() {
                    let blocked = window
                        .get("adsbygoogle")
                        .expect("adsbygoogle")
                        .is_undefined();
                    ad_blocked.set(blocked);
                }
            });
            || ()
        });
    }

    // Initialize ads if they're not blocked
    {
        use_effect(move || {
            if let Some(window) = web_sys::window() {
                let _ = js_sys::Function::new_no_args(
                    "(adsbygoogle = window.adsbygoogle || []).push({});",
                )
                .call0(&JsValue::NULL);
            }
            || ()
        });
    }

    match selected_network {
        Some(AdNetwork::GoogleAdsense {
            client,
            slot,
            width: _,
            height: _,
        }) => {
            if *ad_blocked {
                // If ads are blocked, try to show Buy Me A Coffee if available
                let buy_me_coffee = props
                    .networks
                    .iter()
                    .find(|network| matches!(network, AdNetwork::BuyMeACoffee { .. }));

                match buy_me_coffee {
                    Some(AdNetwork::BuyMeACoffee {
                        data_id,
                        text,
                        button_colour,
                    }) => html! {
                        <div class="advertisement">
                            <div class="advertisement__message">
                                {"Ad blocker detected. Please consider supporting us:"}
                            </div>
                            <div class="advertisement__container">
                                <BuyMeCoffeeComponent
                                    data_id={data_id.clone()}
                                    text={text}
                                    button_colour={button_colour}
                                />
                            </div>
                        </div>
                    },
                    _ => html! {
                        <div class="advertisement">
                            <div class="advertisement__message">
                                {"Ad blocker detected. Please consider disabling it to support us."}
                            </div>
                        </div>
                    },
                }
            } else {
                html! {
                    <div class="advertisement">
                        <div class="advertisement__container">
                            <ins class="adsbygoogle"
                                style="display:block; width:100%; height:320px;"
                                data-ad-client={client}
                                data-ad-slot={slot}
                                data-ad-format="auto"
                                data-full-width-responsive="true">
                            </ins>
                        </div>
                    </div>
                }
            }
        }
        Some(AdNetwork::BuyMeACoffee {
            data_id,
            text,
            button_colour,
        }) => html! {
            <div class="advertisement">
                <div class="advertisement__container">
                    <BuyMeCoffeeComponent
                        data_id={data_id}
                        text={text}
                        button_colour={button_colour}
                    />
                </div>
            </div>
        },
        None => html! {},
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        AdvertisementComponent,
        AdvertisementProps {
            show_probability: 100,
            placement: Some("preview".to_string()),
            networks: vec![
                AdNetwork::GoogleAdsense {
                    client: "ca-pub-5712533029715832".to_string(),
                    slot: "1547914214".to_string(),
                    width: "100vw".to_string(),
                    height: "320".to_string(),
                },
                AdNetwork::BuyMeACoffee {
                    data_id: "chriamue".to_string(),
                    text: "Buy me a coffee",
                    button_colour: "FFDD00",
                },
            ],
        },
        (
            "low_probability",
            AdvertisementProps {
                show_probability: 30,
                placement: Some("preview".to_string()),
                networks: vec![
                    AdNetwork::GoogleAdsense {
                        client: "ca-pub-5712533029715832".to_string(),
                        slot: "1547914214".to_string(),
                        width: "100vw".to_string(),
                        height: "320".to_string(),
                    },
                    AdNetwork::BuyMeACoffee {
                        data_id: "chriamue".to_string(),
                        text: "Buy me a coffee",
                        button_colour: "FFDD00",
                    },
                ],
            }
        )
    );
}
