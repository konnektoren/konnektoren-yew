use super::*;
use async_trait::async_trait;
use futures::channel::oneshot;
use js_sys::Function;
use std::sync::{Arc, RwLock};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/src/components/marketplace/wallet/ton_wallet.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn initTonWallet(
        manifest_url: &str,
        on_connect: JsValue,
        on_disconnect: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn payTonWallet(address: String, amount: u64) -> Result<JsValue, JsValue>;
}

#[derive(Clone)]
pub struct TonWalletProvider {
    connection: Arc<RwLock<Option<WalletConnection>>>,
    manifest_url: String,
    balance: Arc<RwLock<f64>>,
    // Store callbacks so they aren't dropped (wrapped in Arc/RwLock).
    _connect_callback: Arc<RwLock<Option<Arc<Closure<dyn Fn(String, String)>>>>>,
    _disconnect_callback: Arc<RwLock<Option<Arc<Closure<dyn Fn(String)>>>>>,
}

impl PartialEq for TonWalletProvider {
    fn eq(&self, other: &Self) -> bool {
        *self.connection.read().unwrap() == *other.connection.read().unwrap()
            && self.manifest_url == other.manifest_url
            && *self.balance.read().unwrap() == *other.balance.read().unwrap()
    }
}

impl TonWalletProvider {
    pub fn new(manifest_url: String) -> Self {
        Self {
            connection: Arc::new(RwLock::new(None)),
            manifest_url,
            balance: Arc::new(RwLock::new(0.0)),
            _connect_callback: Arc::new(RwLock::new(None)),
            _disconnect_callback: Arc::new(RwLock::new(None)),
        }
    }
}

#[async_trait(?Send)]
impl WalletProvider for TonWalletProvider {
    async fn connect(&self) -> Result<WalletConnection, String> {
        // If we already have a connection, just return it.
        if let Some(existing) = &*self.connection.read().unwrap() {
            return Ok(existing.clone());
        }

        let (tx, rx) = oneshot::channel();
        let tx = Arc::new(RwLock::new(Some(tx)));

        // Create on_connect closure that will handle the connection response
        let on_wallet_connect = Arc::new(Closure::wrap(Box::new({
            let tx = tx.clone();
            move |addr: String, bal: String| {
                if let Some(tx) = tx.write().unwrap().take() {
                    let balance = bal.parse::<u64>().unwrap_or(0) as f64 / 1_000_000_000.0;
                    let connection = WalletConnection {
                        address: addr,
                        network: Network::TonTestnet,
                    };
                    let _ = tx.send((connection, balance));
                }
            }
        }) as Box<dyn Fn(String, String)>));

        // Create on_disconnect closure that will clean up the connection
        let connection = self.connection.clone();
        let balance = self.balance.clone();
        let on_wallet_disconnect = Arc::new(Closure::wrap(Box::new(move |_addr: String| {
            *connection.write().unwrap() = None;
            *balance.write().unwrap() = 0.0;
        }) as Box<dyn Fn(String)>));

        // Store closures in fields so they remain alive
        {
            let mut cb_connect = self._connect_callback.write().unwrap();
            *cb_connect = Some(on_wallet_connect.clone());
            let mut cb_disconnect = self._disconnect_callback.write().unwrap();
            *cb_disconnect = Some(on_wallet_disconnect.clone());
        }

        // Initialize the TON wallet
        match initTonWallet(
            &self.manifest_url,
            on_wallet_connect
                .as_ref()
                .as_ref()
                .unchecked_ref::<Function>()
                .into(),
            on_wallet_disconnect
                .as_ref()
                .as_ref()
                .unchecked_ref::<Function>()
                .into(),
        )
        .await
        {
            Ok(_) => {
                // Wait for the connection response
                match rx.await {
                    Ok((connection, balance)) => {
                        *self.connection.write().unwrap() = Some(connection.clone());
                        *self.balance.write().unwrap() = balance;
                        Ok(connection)
                    }
                    Err(_) => Err("Failed to receive wallet connection".to_string()),
                }
            }
            Err(e) => Err(format!("Failed to initialize wallet: {:?}", e)),
        }
    }

    async fn disconnect(&self) -> Result<WalletConnection, String> {
        let connection = self.connection.read().unwrap().clone();
        if let Some(conn) = connection {
            *self.connection.write().unwrap() = None;
            *self.balance.write().unwrap() = 0.0;
            Ok(conn)
        } else {
            Err("Wallet not connected".to_string())
        }
    }

    async fn get_balance(&self, token: &Token) -> Result<WalletBalance, String> {
        if token.token_type != TokenType::Native {
            return Err("Only native TON token is supported".to_string());
        }
        if let Some(_connection) = &*self.connection.read().unwrap() {
            Ok(WalletBalance {
                amount: *self.balance.read().unwrap(),
                token: token.clone(),
            })
        } else {
            Err("Wallet not connected".to_string())
        }
    }

    async fn send_payment(
        &self,
        _connection: &WalletConnection,
        recipient: &str,
        amount: u64,
        token: &Token,
    ) -> Result<String, String> {
        if token.token_type != TokenType::Native {
            return Err("Only native TON token is supported".to_string());
        }
        if self.connection.read().unwrap().is_none() {
            return Err("Wallet not connected".to_string());
        }
        match payTonWallet(recipient.to_string(), amount).await {
            Ok(result) => Ok(format!("Transaction successful: {:?}", result)),
            Err(e) => Err(format!("Payment failed: {:?}", e)),
        }
    }

    fn is_connected(&self) -> bool {
        self.connection.read().unwrap().is_some()
    }

    fn get_network(&self) -> Network {
        Network::TonTestnet
    }

    fn render_ui(
        &self,
        connection: Option<WalletConnection>,
        on_connect: Option<yew::Callback<WalletConnection>>,
        on_update: Option<Callback<Self>>,
    ) -> Html {
        html! {
            <div class="wallet__connect-wrapper">
                // This div must always be present for the TonConnect UI
                <div id="ton-wallet-button" class="ton-wallet__button" />

                if let Some(_conn) = connection {
                    <div class="wallet__connected-message">
                        { "TON Wallet connected!" }
                    </div>
                } else {
                    <button class="wallet__connect-button" onclick={
                        let on_connect = on_connect.clone();
                        let on_update = on_update.clone();
                        let provider = self.clone();
                        Callback::from(move |_| {
                            let provider = provider.clone();
                            let on_connect = on_connect.clone();
                            let on_update = on_update.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                match provider.connect().await {
                                    Ok(conn) => {
                                        if let Some(cb) = on_connect {
                                            cb.emit(conn);
                                        }
                                    }
                                    Err(e) => {
                                        web_sys::console::error_1(&format!("Connect error: {e}").into());
                                    }
                                }
                                if let Some(cb) = on_update {
                                    cb.emit(provider);
                                }
                            });
                        })
                    }>
                        <i class="fas fa-power-off" />
                        { "Connect TON Wallet" }
                    </button>
                }
            </div>
        }
    }

    async fn fetch_token_price(&self, token: &Token) -> Result<f64, String> {
        if token.token_type != TokenType::Native {
            return Err("Only native TON token is supported".to_string());
        }
        let url = "https://testnet.tonapi.io/v2/rates?tokens=ton&currencies=usd";
        let response = gloo::net::http::Request::get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
        json["rates"]["TON"]["prices"]["USD"]
            .as_f64()
            .ok_or("Failed to parse TON price".to_string())
    }
}
