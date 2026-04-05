use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
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
    use yew_preview::test_utils::{exists, has_attribute, has_class, has_text};

    yew_preview::create_preview_with_tests!(
        component: BuyMeCoffeeComponent,
        default_props: BuyMeCoffeeProps {
            data_id: "chriamue".to_string(),
            text: "Buy me a coffee",
            button_colour: "FFDD00",
            font_colour: "000000",
            font_family: "Cookie",
            outline_colour: "000000",
            coffee_colour: "ffffff",
        },
        variants: [
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
            ),
            (
                "dark",
                BuyMeCoffeeProps {
                    data_id: "chriamue".to_string(),
                    text: "Buy me a coffee",
                    button_colour: "222222",
                    font_colour: "ffffff",
                    font_family: "Cookie",
                    outline_colour: "ffffff",
                    coffee_colour: "FFDD00",
                }
            ),
            (
                "purple",
                BuyMeCoffeeProps {
                    data_id: "chriamue".to_string(),
                    text: "Sponsor this project",
                    button_colour: "7B2FBE",
                    font_colour: "ffffff",
                    font_family: "Poppins",
                    outline_colour: "7B2FBE",
                    coffee_colour: "ffffff",
                }
            ),
        ],
        tests: [
            ("Has buy-me-coffee wrapper", has_class("buy-me-coffee")),
            ("Has link to buymeacoffee", has_text("buymeacoffee.com/chriamue")),
            ("Link opens in new tab", has_attribute("target", "_blank")),
            ("Has button image", exists("<img")),
            ("Image src uses slug", has_text("slug=chriamue")),
            ("Image src uses button colour", has_text("button_colour=FFDD00")),
        ]
    );
}
