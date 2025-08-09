use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BuyMeCoffeeProps {
    pub data_id: String,
    #[prop_or("Buy me a coffee")]
    pub text: &'static str,
    #[prop_or("FFDD00")]
    pub button_colour: &'static str,
    #[prop_or("000000")]
    pub font_colour: &'static str,
    #[prop_or("Cookie")]
    pub font_family: &'static str,
    #[prop_or("000000")]
    pub outline_colour: &'static str,
    #[prop_or("ffffff")]
    pub coffee_colour: &'static str,
}

#[function_component(BuyMeCoffeeComponent)]
pub fn buy_me_coffee(props: &BuyMeCoffeeProps) -> Html {
    let img_url = format!(
        "https://img.buymeacoffee.com/button-api/?text={}&emoji=&slug={}&button_colour={}&font_colour={}&font_family={}&outline_colour={}&coffee_colour={}",
        urlencoding::encode(props.text),
        props.data_id,
        props.button_colour,
        props.font_colour,
        props.font_family,
        props.outline_colour,
        props.coffee_colour
    );

    let link_url = format!("https://www.buymeacoffee.com/{}", props.data_id);

    html! {
        <div class="buy-me-coffee">
            <a href={link_url} target="_blank" rel="noopener">
                <img src={img_url} alt="Buy Me A Coffee" />
            </a>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        BuyMeCoffeeComponent,
        BuyMeCoffeeProps {
            data_id: "chriamue".to_string(),
            text: "Buy me a coffee",
            button_colour: "FFDD00",
            font_colour: "000000",
            font_family: "Cookie",
            outline_colour: "000000",
            coffee_colour: "ffffff",
        },
        (
            "custom",
            BuyMeCoffeeProps {
                data_id: "chriamue".to_string(),
                text: "Support my work",
                button_colour: "40DCA5",
                font_colour: "ffffff",
                font_family: "Lato",
                outline_colour: "000000",
                coffee_colour: "ffffff",
            }
        )
    );
}
