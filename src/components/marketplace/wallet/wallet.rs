use super::token_selector::TokenSelector;
use super::*;
use std::collections::HashMap;
use yew::prelude::*;

fn truncate_address(address: &str, show_full: bool) -> String {
    if address.len() <= 6 {
        return address.to_string();
    }

    if show_full {
        address.to_string()
    } else {
        format!(
            "{}...{}",
            &address[..6],
            &address[address.len().saturating_sub(4)..]
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct WalletComponentProps<T: WalletProvider + 'static> {
    pub provider: T,
    pub tokens: Vec<Token>,
    #[prop_or_default]
    pub price: Option<f64>,
    #[prop_or_default]
    pub recipient_address: Option<String>,
    #[prop_or_default]
    pub on_payment_success: Option<Callback<()>>,
    #[prop_or_default]
    pub on_connect: Option<Callback<WalletConnection>>,
    #[prop_or_default]
    pub on_disconnect: Option<Callback<WalletConnection>>,
    #[prop_or_default]
    pub on_token_select: Option<Callback<Token>>,
}

#[function_component(WalletComponent)]
pub fn wallet_component<T: WalletProvider + 'static>(props: &WalletComponentProps<T>) -> Html {
    let show_full_address = use_state(|| false);
    let provider = use_state(|| props.provider.clone());
    let connection = use_state(|| None::<WalletConnection>);
    let selected_token = use_state(|| props.tokens.first().cloned());
    let token_balances = use_state(HashMap::<String, WalletBalance>::new);
    let token_price = use_state(|| None::<f64>);
    let error_state = use_state(|| None::<String>);
    let is_sending = use_state(|| false);

    {
        let provider = provider.clone();
        let selected_token = selected_token.clone();
        let token_price = token_price.clone();

        use_effect_with((*selected_token).clone(), move |token| {
            if let Some(token) = token {
                let provider = provider.clone();
                let token_price = token_price.clone();
                let token = token.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(price) = provider.fetch_token_price(&token).await {
                        token_price.set(Some(price));
                    }
                });
            }
            || ()
        });
    }

    {
        let provider = provider.clone();
        let connection = connection.clone();
        let token_balances = token_balances.clone();
        let tokens = props.tokens.clone();

        use_effect_with((), move |_| {
            let provider = provider.clone();
            let connection = connection.clone();
            let token_balances = token_balances.clone();
            let tokens = tokens.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let provider_instance = (*provider).clone();
                provider.set(provider_instance.clone());

                // Update balances if already connected
                if let Some(_conn) = &*connection {
                    for token in &tokens {
                        if let Ok(balance) = provider_instance.get_balance(token).await {
                            token_balances.set(
                                (*token_balances)
                                    .clone()
                                    .into_iter()
                                    .chain(std::iter::once((
                                        token.contract_address.clone(),
                                        balance,
                                    )))
                                    .collect(),
                            );
                        }
                    }
                }
            });
            || ()
        });
    }

    let on_payment = {
        let provider = provider.clone();
        let selected_token = selected_token.clone();
        let token_price = token_price.clone();
        let on_payment_success = props.on_payment_success.clone();
        let price = props.price;
        let recipient_address = props.recipient_address.clone();
        let error_state = error_state.clone();
        let is_sending = is_sending.clone();
        let connection = connection.clone();

        Callback::from(move |_| {
            if let (Some(token), Some(price), Some(token_price)) =
                ((*selected_token).clone(), price, *token_price)
            {
                if let (Some(connection), Some(recipient)) =
                    ((*connection).clone(), recipient_address.clone())
                {
                    let provider = (*provider).clone();
                    let on_success = on_payment_success.clone();
                    let error_state = error_state.clone();
                    let is_sending = is_sending.clone();

                    let amount = match token.token_type {
                        TokenType::Native => {
                            // Convert to lamports/nanotons
                            (price / token_price * 1_000_000_000.0) as u64
                        }
                        TokenType::Custom => {
                            // Convert to token decimal places
                            (price * 10f64.powi(token.decimals as i32)) as u64
                        }
                    };

                    is_sending.set(true);
                    error_state.set(None);

                    wasm_bindgen_futures::spawn_local(async move {
                        match provider
                            .send_payment(&connection, &recipient, amount, &token)
                            .await
                        {
                            Ok(_) => {
                                if let Some(callback) = on_success {
                                    callback.emit(());
                                }
                                error_state.set(None);
                            }
                            Err(e) => {
                                log::error!("Payment failed: {}", e);
                                error_state.set(Some(e));
                            }
                        }
                        is_sending.set(false);
                    });
                }
            }
        })
    };

    let on_token_select = {
        let selected_token = selected_token.clone();
        let on_token_select = props.on_token_select.clone();
        let token_price = token_price.clone();
        let provider = provider.clone();

        Callback::from(move |token: Token| {
            selected_token.set(Some(token.clone()));

            let provider = provider.clone();
            let token_price = token_price.clone();
            let token_for_price = token.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(price) = provider.fetch_token_price(&token_for_price).await {
                    token_price.set(Some(price));
                }
            });

            if let Some(callback) = &on_token_select {
                callback.emit(token);
            }
        })
    };

    let on_disconnect = {
        let provider = provider.clone();
        let connection = connection.clone();
        let on_disconnect = props.on_disconnect.clone();
        let error_state = error_state.clone();

        Callback::from(move |_| {
            let provider = provider.clone();
            let connection = connection.clone();
            let on_disconnect = on_disconnect.clone();
            let error_state = error_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Disconnecting wallet");
                match provider.disconnect().await {
                    Ok(disconnected) => {
                        provider.set((*provider).clone());
                        connection.set(None);
                        error_state.set(None);
                        if let Some(callback) = on_disconnect {
                            callback.emit(disconnected);
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to disconnect wallet: {}", e);
                        error_state.set(Some(format!("Failed to disconnect wallet: {}", e)));
                    }
                }
            });
        })
    };

    let on_wallet_connect = {
        let connection = connection.clone();
        let provider = provider.clone();
        let on_connect = props.on_connect.clone();
        let token_balances = token_balances.clone();
        let tokens = props.tokens.clone();

        Callback::from(move |new_connection: WalletConnection| {
            connection.set(Some(new_connection.clone()));

            if let Some(callback) = on_connect.clone() {
                callback.emit(new_connection.clone());
            }

            // Update balances after connection
            wasm_bindgen_futures::spawn_local({
                let provider = provider.clone();
                let token_balances = token_balances.clone();
                let tokens = tokens.clone();

                async move {
                    for token in &tokens {
                        if let Ok(balance) = provider.get_balance(token).await {
                            token_balances.set(
                                (*token_balances)
                                    .clone()
                                    .into_iter()
                                    .chain(std::iter::once((
                                        token.contract_address.clone(),
                                        balance,
                                    )))
                                    .collect(),
                            );
                        }
                    }
                }
            });
        })
    };

    let on_provider_update = {
        let provider = provider.clone();

        Callback::from(move |new_provider: T| {
            provider.set(new_provider);
        })
    };

    html! {
        <div class="wallet">
            <div class="wallet__container">
                <div class="wallet__header">
                    <h2>{format!("{} Wallet", provider.get_network())}</h2>
                    <div class="wallet__network-badge">
                        {props.provider.get_network().to_string()}
                    </div>
                </div>
                <div class="wallet__content">
                    // Always render the provider UI first
                    <div class="wallet__ui-section">
                        {provider.render_ui((*connection).clone(), Some(on_wallet_connect.clone()), Some(on_provider_update.clone()))}
                    </div>

                    // Then render the rest of the UI when connected
                    if let Some(conn) = &*connection {
                        <div class="wallet__connected-content">
                            <div class="wallet__status">
                                <div class="address-display">
                                    <p class="address-display__text">
                                        {"Address: "}
                                        {truncate_address(&conn.address, *show_full_address)}
                                    </p>
                                    <button
                                        class="address-display__button"
                                        onclick={
                                            let show_full_address = show_full_address.clone();
                                            Callback::from(move |_| {
                                                show_full_address.set(!*show_full_address);
                                            })
                                        }
                                    >
                                        if *show_full_address {
                                            <i class="fas fa-eye-slash" />
                                        } else {
                                            <i class="fas fa-eye" />
                                        }
                                    </button>
                                </div>
                                if let Some(selected) = &*selected_token {
                                    if let Some(balance) = (*token_balances).get(&selected.contract_address) {
                                        <p class="balance-display">
                                            {"Balance: "}{format!("{:.4} {}", balance.amount, balance.token.symbol)}
                                        </p>
                                    }
                                }
                            </div>

                            <TokenSelector
                                tokens={props.tokens.clone()}
                                selected_token={(*selected_token).clone()}
                                on_select={on_token_select.clone()}
                            />

                            {
                                if let (Some(price), Some(token_price), Some(selected)) = (props.price, *token_price, &*selected_token) {
                                    let amount = match selected.token_type {
                                        TokenType::Native => price / token_price,
                                        TokenType::Custom => price,
                                    };
                                    html! {
                                        <>
                                            if let Some(error) = (*error_state).clone() {
                                                <div class="wallet__error">
                                                    {error}
                                                </div>
                                            }
                                            <button
                                                class={classes!(
                                                    "wallet__payment-button",
                                                    match selected.network {
                                                        #[cfg(feature = "csr")]
                                                        Network::TonTestnet => "ton",
                                                        #[cfg(feature = "solana")]
                                                        Network::SolanaDevnet => "solana",
                                                        _ => ""
                                                    },
                                                    if *is_sending { "loading" } else { "" }
                                                )}
                                                onclick={on_payment.clone()}
                                                disabled={*is_sending}
                                            >
                                                if *is_sending {
                                                    {"Sending..."}
                                                } else {
                                                    {format!("Pay {:.4} {}", amount, selected.symbol)}
                                                }
                                            </button>
                                        </>
                                    }
                                } else {
                                    html! {}
                                }
                            }

                            <button class="wallet__disconnect" onclick={on_disconnect}>
                                {"Disconnect"}
                            </button>
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    #[cfg(feature = "solana")]
    use crate::components::marketplace::wallet::solana::SolanaWalletProvider;
    #[cfg(feature = "csr")]
    use crate::components::marketplace::wallet::ton::TonWalletProvider;
    use yew_preview::prelude::*;

    fn get_testnet_tokens() -> Vec<Token> {
        #[cfg(feature = "csr")]
        {
            vec![
                Token {
                    name: "Toncoin".to_string(),
                    symbol: "TON".to_string(),
                    image_url: "https://cryptologos.cc/logos/toncoin-ton-logo.svg".to_string(),
                    contract_address:
                        "0:5ca1f07c7d67fd26816a731377b6404e857265761676626a4bd6fda652293119"
                            .to_string(),
                    decimals: 9,
                    network: Network::TonTestnet,
                    token_type: TokenType::Native,
                },
                Token {
                    name: "Pepe TON".to_string(),
                    symbol: "PEPETON".to_string(),
                    image_url: "https://ton-pepe.com/pepe.png".to_string(),
                    contract_address: "EQCD39VS5jcptHL8vMjEXrzGaRcCVYto7HUn4bpAOg8xqB2N"
                        .to_string(),
                    decimals: 9,
                    network: Network::TonTestnet,
                    token_type: TokenType::Custom,
                },
            ]
        }
        #[cfg(not(feature = "csr"))]
        {
            vec![]
        }
    }

    #[cfg(feature = "solana")]
    fn get_devnet_tokens() -> Vec<Token> {
        vec![
            Token {
                name: "Solana".to_string(),
                symbol: "SOL".to_string(),
                image_url: "https://cryptologos.cc/logos/solana-sol-logo.svg"
                    .to_string(),
                contract_address: "11111111111111111111111111111111".to_string(),
                decimals: 9,
                network: Network::SolanaDevnet,
                token_type: TokenType::Native,
            },
            Token {
                name: "Melania Token".to_string(),
                symbol: "MELANIA".to_string(),
                image_url: "https://melaniameme.com/hero.jpg".to_string(),
                contract_address: "FUAfBo2jgks6gB4Z4LfZkqSZgzNucisEHqnNebaRxM1P".to_string(),
                decimals: 9,
                network: Network::SolanaDevnet,
                token_type: TokenType::Custom,
            },
            Token {
                name: "OFFICIAL TRUMP".to_string(),
                symbol: "TRUMP".to_string(),
                image_url: "https://gettrumpmemes.com/images/TrumpCard_EnjoyMySneakers_Signiture-680_1.webp".to_string(),
                contract_address: "6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN".to_string(),
                decimals: 9,
                network: Network::SolanaDevnet,
                token_type: TokenType::Custom,
            }
        ]
    }

    #[cfg(feature = "csr")]
    pub type TonWallet = WalletComponent<TonWalletProvider>;
    #[cfg(feature = "solana")]
    pub type SolanaWallet = WalletComponent<SolanaWalletProvider>;

    #[cfg(feature = "csr")]
    yew_preview::create_preview!(
        TonWallet,
        WalletComponentProps {
            provider: TonWalletProvider::new(
                "https://konnektoren.help/assets/tonconnect-manifest.json".to_string()
            ),
            tokens: get_testnet_tokens(),
            price: Some(10.0),
            recipient_address: Some(
                "0:5ca1f07c7d67fd26816a731377b6404e857265761676626a4bd6fda652293119".to_string()
            ),
            on_payment_success: Some(Callback::from(|_| {
                log::info!("Payment successful");
            })),
            on_connect: Some(Callback::from(|conn| {
                log::info!("TON Connected: {:?}", conn);
            })),
            on_disconnect: Some(Callback::from(|_| {
                log::info!("TON Disconnected");
            })),
            on_token_select: Some(Callback::from(|token| {
                log::info!("TON Selected token: {:?}", token);
            })),
        },
    );

    #[cfg(feature = "solana")]
    yew_preview::create_preview!(
        SolanaWallet,
        WalletComponentProps {
            provider: SolanaWalletProvider::new(Network::SolanaDevnet),
            tokens: get_devnet_tokens(),
            price: Some(10.0),
            recipient_address: Some("4Qsvf6oNaAHNpwzV2RZfnqSpMd1oP6LXgRxNT5xd72Nj".to_string()),
            on_payment_success: Some(Callback::from(|_| {
                log::info!("Payment successful");
            })),
            on_connect: Some(Callback::from(|conn| {
                log::info!("Solana Connected: {:?}", conn);
            })),
            on_disconnect: Some(Callback::from(|_| {
                log::info!("Solana Disconnected");
            })),
            on_token_select: Some(Callback::from(|token| {
                log::info!("Solana Selected token: {:?}", token);
            })),
        },
    );
}
