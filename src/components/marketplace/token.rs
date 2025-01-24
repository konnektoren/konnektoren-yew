use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Native,
    SPL,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub image_url: String,
    pub contract_address: String,
    pub decimals: u8,
    pub network: TokenNetwork,
    pub token_type: TokenType,
}

impl Token {
    pub fn native_sol() -> Self {
        Self {
            name: "Solana".to_string(),
            symbol: "SOL".to_string(),
            image_url: "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png".to_string(),
            contract_address: "So11111111111111111111111111111111111111112".to_string(),
            decimals: 9,
            network: TokenNetwork::Solana,
            token_type: TokenType::Native,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenNetwork {
    Solana,
    Ethereum,
    // Add other networks as needed
}

#[derive(Properties, PartialEq)]
pub struct TokenSelectorProps {
    pub tokens: Vec<Token>,
    pub selected_token: Option<Token>,
    pub on_select: Callback<Token>,
}

#[function_component(TokenSelector)]
pub fn token_selector(props: &TokenSelectorProps) -> Html {
    html! {
        <div class="token-selector">
            <div class="token-selector__list">
                {
                    props.tokens.iter().map(|token| {
                        let is_selected = props.selected_token.as_ref() == Some(token);
                        let on_click = {
                            let token = token.clone();
                            let on_select = props.on_select.clone();
                            Callback::from(move |_| {
                                on_select.emit(token.clone());
                            })
                        };

                        html! {
                            <button
                                onclick={on_click}
                                class={classes!(
                                    "token-selector__button",
                                    if is_selected { "selected" } else { "" }
                                )}
                            >
                                <TokenDisplay token={token.clone()} />
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}

#[function_component(TokenDisplay)]
pub fn token_display(props: &TokenDisplayProps) -> Html {
    html! {
        <div class="token-display">
            <div class="token-display__image">
                <img src={props.token.image_url.clone()} alt={format!("{} icon", props.token.symbol)} />
            </div>
            <div class="token-display__info">
                <span class="token-display__name">{&props.token.name}</span>
                <span class="token-display__symbol">{&props.token.symbol}</span>
            </div>
            if let Some(balance) = props.balance {
                <span class="token-display__balance">
                    {format!("{:.4} {}", balance, props.token.symbol)}
                </span>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TokenDisplayProps {
    pub token: Token,
    #[prop_or_default]
    pub balance: Option<f64>,
}
