use super::Token;
use yew::prelude::*;

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
                {for props.tokens.iter().map(|token| {
                    let selected = props.selected_token.as_ref() == Some(token);
                    let on_click = {
                        let token = token.clone();
                        let on_select = props.on_select.clone();
                        Callback::from(move |_| on_select.emit(token.clone()))
                    };
                    html! {
                        <button
                            class={classes!("token-selector__button", if selected { "selected" } else { "" })}
                            onclick={on_click}
                        >
                            <div class="token-selector__token-display">
                                <div class="token-selector__image">
                                    <img src={token.image_url.clone()} alt={token.symbol.clone()} />
                                </div>
                                <div class="token-selector__info">
                                    <span class="token-selector__name">{&token.name}</span>
                                    <span class="token-selector__symbol">{&token.symbol}</span>
                                </div>
                            </div>
                        </button>
                    }
                })}
            </div>
        </div>
    }
}
