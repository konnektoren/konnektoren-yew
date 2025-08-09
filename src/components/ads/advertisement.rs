use crate::components::{
    BlogAdComponent, BuyMeCoffeeComponent, GumroadSubscribeComponent, VideoComponent,
};
use crate::i18n::use_i18n;
use gloo::timers::callback::Interval;
use rand::Rng;
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
    GumroadSubscribe {
        subscribe_url: String,
    },
    VideoAd {
        src: String,
        preview: Option<String>,
        title: Option<String>,
        autoplay: bool,
    },
    BlogAd {
        blog_url: String,
        title: Option<String>,
        preview_text: Option<String>,
        preview_image: Option<String>,
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

fn select_random_ad(networks: &[AdNetwork], show_probability: u8) -> Option<AdNetwork> {
    if networks.is_empty() {
        return None;
    }

    let mut rng = rand::thread_rng();
    if rng.gen_range(0..100) < show_probability {
        let index = rng.gen_range(0..networks.len());
        Some(networks[index].clone())
    } else {
        None
    }
}

#[function_component(AdvertisementComponent)]
pub fn advertisement(props: &AdvertisementProps) -> Html {
    let i18n = use_i18n();

    // State to hold the currently selected ad
    let selected_network = use_state(|| select_random_ad(&props.networks, props.show_probability));

    // State to handle transition fade
    let is_fading = use_state(|| false);
    let ad_blocked = use_state(|| false);

    // Set up interval to rotate ads every 20 seconds with transition
    {
        let selected_network = selected_network.clone();
        let is_fading = is_fading.clone();
        let networks = props.networks.clone();
        let show_probability = props.show_probability;

        use_effect_with((), move |_| {
            let interval = Interval::new(20_000, move || {
                // Start fade out
                is_fading.set(true);

                // After 300ms (fade out duration), change ad and fade in
                let selected_network = selected_network.clone();
                let is_fading = is_fading.clone();
                let networks = networks.clone();

                gloo::timers::callback::Timeout::new(300, move || {
                    let new_ad = select_random_ad(&networks, show_probability);
                    selected_network.set(new_ad);
                    is_fading.set(false);
                })
                .forget();
            });

            move || drop(interval)
        });
    }

    #[cfg(feature = "csr")]
    {
        let ad_blocked = ad_blocked.clone();
        use_effect_with((), move |_| {
            use gloo::timers::future::TimeoutFuture;
            use wasm_bindgen::JsValue;
            use web_sys::window;

            wasm_bindgen_futures::spawn_local(async move {
                TimeoutFuture::new(2000).await;
                if let Some(window) = window() {
                    let blocked = window
                        .get("adsbygoogle")
                        .expect("adsbygoogle")
                        .is_undefined();
                    ad_blocked.set(blocked);
                }
            });
            || ()
        });

        use_effect(move || {
            use wasm_bindgen::prelude::*;
            use web_sys::window;

            if let Some(_window) = window() {
                let _ = js_sys::Function::new_no_args(
                    "(adsbygoogle = window.adsbygoogle || []).push({});",
                )
                .call0(&JsValue::NULL);
            }
            || ()
        });
    }

    let ad_class = classes!(
        "advertisement",
        if *is_fading {
            "advertisement--fading"
        } else {
            ""
        }
    );

    let ad_info_label = html! {
        <div class="advertisement__info">
            <span class="advertisement__label">{ i18n.t("Advertisement") }</span>
            <div class="advertisement__timer">
                <div class="advertisement__timer-bar"></div>
            </div>
        </div>
    };

    match &*selected_network {
        Some(AdNetwork::GoogleAdsense {
            client,
            slot,
            width: _,
            height: _,
        }) => {
            if *ad_blocked {
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
                        <div class={ad_class}>
                            {ad_info_label}
                            <div class="advertisement__message">
                                { i18n.t("Ad blocker detected. Please consider supporting us:") }
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
                        <div class={ad_class}>
                            {ad_info_label}
                            <div class="advertisement__message">
                                { i18n.t("Ad blocker detected. Please consider disabling it to support us.") }
                            </div>
                        </div>
                    },
                }
            } else {
                #[cfg(feature = "csr")]
                {
                    html! {
                        <div class={ad_class}>
                            {ad_info_label}
                            <div class="advertisement__container">
                                <ins class="adsbygoogle"
                                    style="display:block; width:100%; height:320px;"
                                    data-ad-client={client.clone()}
                                    data-ad-slot={slot.clone()}
                                    data-ad-format="auto"
                                    data-full-width-responsive="true">
                                </ins>
                            </div>
                        </div>
                    }
                }
                #[cfg(not(feature = "csr"))]
                {
                    html! {}
                }
            }
        }
        Some(AdNetwork::BuyMeACoffee {
            data_id,
            text,
            button_colour,
        }) => html! {
            <div class={ad_class}>
                {ad_info_label}
                <div class="advertisement__container">
                    <BuyMeCoffeeComponent
                        data_id={data_id.clone()}
                        text={text}
                        button_colour={button_colour}
                    />
                </div>
            </div>
        },
        Some(AdNetwork::GumroadSubscribe { subscribe_url }) => html! {
            <div class={ad_class}>
                {ad_info_label}
                <div class="advertisement__container">
                    <GumroadSubscribeComponent
                        subscribe_url={subscribe_url.clone()}
                    />
                </div>
            </div>
        },
        Some(AdNetwork::VideoAd {
            src,
            preview,
            title,
            autoplay,
        }) => html! {
            <div class={ad_class}>
                {ad_info_label}
                <div class="advertisement__container">
                    <VideoComponent
                        src={src.clone()}
                        preview={preview.clone()}
                        title={title.clone()}
                        autoplay={*autoplay}
                    />
                </div>
            </div>
        },
        Some(AdNetwork::BlogAd {
            blog_url,
            title,
            preview_text,
            preview_image,
        }) => html! {
            <div class={ad_class}>
                {ad_info_label}
                <div class="advertisement__container">
                    <BlogAdComponent
                        blog_url={blog_url.clone()}
                        title={title.clone()}
                        preview_text={preview_text.clone()}
                        preview_image={preview_image.clone()}
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
                AdNetwork::GumroadSubscribe {
                    subscribe_url: "https://konnektoren.gumroad.com/subscribe".to_string(),
                },
                AdNetwork::VideoAd {
                    src: "https://youtu.be/ZiZibuHyU_k".to_string(),
                    preview: Some(
                        "https://img.youtube.com/vi/ZiZibuHyU_k/hqdefault.jpg".to_string()
                    ),
                    title: Some("Konnektoren Video Ad".to_string()),
                    autoplay: false,
                },
                AdNetwork::BlogAd {
                    blog_url: "https://www.konnektoren.blog/".to_string(),
                    title: Some("Konnektoren Blog".to_string()),
                    preview_text: Some("Discover language learning tips, cultural insights, and educational resources.".to_string()),
                    preview_image: Some("https://konnektoren.help/favicon.png".to_string()),
                },
            ],
        },
    );
}
