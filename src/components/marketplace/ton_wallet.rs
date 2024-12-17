use std::rc::Rc;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[wasm_bindgen(module = "/src/components/marketplace/ton_wallet.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn initTonWallet(
        manifest_url: &str,
        on_connect: &WalletConnectCallback,
        on_disconnect: &WalletDisconnectCallback,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn payTonWallet(address: Address, amount: NanoTon) -> Result<JsValue, JsValue>;
}

pub type WalletConnectCallback = Closure<dyn Fn(Address, String)>;
pub type WalletDisconnectCallback = Closure<dyn Fn(Address)>;
pub type Address = String;
pub type NanoTon = u64;

#[derive(Clone, PartialEq, Properties, Default)]
pub struct TonWalletProps {
    #[prop_or_default]
    pub on_connect: Option<Callback<Address>>,
    #[prop_or_default]
    pub on_disconnect: Option<Callback<Address>>,
    pub manifest_url: String,
}

#[function_component(TonWalletComponent)]
pub fn ton_wallet_component(props: &TonWalletProps) -> Html {
    let address = use_state(|| "Not connected".to_string());
    let balance = use_state(|| "0".to_string());

    {
        let address = address.clone();
        let balance = balance.clone();
        let on_connect = props.on_connect.clone();
        let on_disconnect = props.on_disconnect.clone();
        let manifest_url = props.manifest_url.clone();

        use_effect_with((), move |_| {
            let address_connect = address.clone();
            let balance_connect = balance.clone();
            let on_wallet_connect = Closure::wrap(Box::new(move |addr: Address, bal: String| {
                log::info!("Connected in rs: {} {}", addr, bal);
                address_connect.set(addr.clone());
                let bal = bal.parse::<u64>().unwrap_or(0);
                balance_connect.set(bal.checked_div(1_000_000_000).unwrap_or(0).to_string());
                if let Some(on_connect) = on_connect.as_ref() {
                    on_connect.emit(addr);
                }
            }) as Box<dyn Fn(String, String)>);

            let address_disconnect = address.clone();
            let balance_disconnect = balance.clone();
            let on_wallet_disconnect = Closure::wrap(Box::new(move |addr: Address| {
                log::info!("Disconnected in rs: {}", addr);
                address_disconnect.set("Not connected".to_string());
                balance_disconnect.set("0".to_string());
                if let Some(on_disconnect) = on_disconnect.as_ref() {
                    on_disconnect.emit(addr);
                }
            }) as Box<dyn Fn(String)>);

            let on_wallet_connect = Rc::new(on_wallet_connect);
            let on_wallet_disconnect = Rc::new(on_wallet_disconnect);

            wasm_bindgen_futures::spawn_local({
                let on_wallet_connect = on_wallet_connect.clone();
                let on_wallet_disconnect = on_wallet_disconnect.clone();
                async move {
                    match initTonWallet(&manifest_url, &on_wallet_connect, &on_wallet_disconnect)
                        .await
                    {
                        Ok(_) => log::info!("Wallet connection initiated"),
                        Err(e) => log::error!("Wallet connection error: {:?}", e),
                    }
                }
            });

            move || {
                drop(on_wallet_connect);
                drop(on_wallet_disconnect);
            }
        });
    }

    html! {
        <div class="ton-wallet">
            <div id="ton-wallet-button" />
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        TonWalletComponent,
        TonWalletProps {
            manifest_url: "https://konnektoren.help/assets/tonconnect-manifest.json".to_string(),
            ..Default::default()
        },
    );
}
