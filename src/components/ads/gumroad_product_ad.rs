use crate::i18n::use_i18n;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct GumroadProductAdProps {
    pub product_url: String,
    pub product_name: String,
    #[prop_or_default]
    pub description: Option<String>,
    #[prop_or_default]
    pub price: Option<String>,
    #[prop_or_default]
    pub preview_image: Option<String>,
    #[prop_or_default]
    pub discount: Option<String>,
}

#[function_component(GumroadProductAdComponent)]
pub fn gumroad_product_ad_component(props: &GumroadProductAdProps) -> Html {
    let i18n = use_i18n();

    let default_description =
        i18n.t("Enhance your learning with our premium educational resources.");
    let description = props.description.as_ref().unwrap_or(&default_description);

    html! {
        <div class="gumroad-product-ad">
            if let Some(ref discount) = props.discount {
                <div class="gumroad-product-ad__badge">
                    <span class="gumroad-product-ad__discount">{ discount }</span>
                </div>
            }

            <div class="gumroad-product-ad__header">
                <div class="gumroad-product-ad__icon">
                    {"📚"}
                </div>
                <div class="gumroad-product-ad__title">
                    { &props.product_name }
                </div>
            </div>

            if let Some(ref image_url) = props.preview_image {
                <div class="gumroad-product-ad__image">
                    <img src={image_url.clone()} alt="Product preview" />
                </div>
            }

            <div class="gumroad-product-ad__content">
                <p class="gumroad-product-ad__description">
                    { description }
                </p>

                if let Some(ref price) = props.price {
                    <div class="gumroad-product-ad__price">
                        { price }
                    </div>
                }
            </div>

            <div class="gumroad-product-ad__actions">
                <a
                    class="gumroad-product-ad__button"
                    href={props.product_url.clone()}
                    target="_blank"
                    rel="noopener"
                >
                    { i18n.t("Get it now") }
                    <span class="gumroad-product-ad__arrow">{"→"}</span>
                </a>
            </div>

            <div class="gumroad-product-ad__footer">
                <span class="gumroad-product-ad__powered">{ i18n.t("Powered by Gumroad") }</span>
                <span class="gumroad-product-ad__secure">{ "🔒 " }{ i18n.t("Secure checkout") }</span>
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        GumroadProductAdComponent,
        GumroadProductAdProps {
            product_url: "https://konnektoren.gumroad.com/l/book-vocabulary-a1-en".to_string(),
            product_name: "Book Vocabulary A1 EN".to_string(),
            description: Some("Master essential English vocabulary with our comprehensive A1 level guide. Perfect for beginners!".to_string()),
            price: Some("$9.99".to_string()),
            preview_image: Some("https://public-files.gumroad.com/d1mh6dq92jyw0dd3b86qu26g3514".to_string()),
            discount: Some("20% OFF".to_string()),
        },
        (
            "minimal",
            GumroadProductAdProps {
                product_url: "https://konnektoren.gumroad.com/l/book-vocabulary-a1-en".to_string(),
                product_name: "Book Vocabulary A1 EN".to_string(),
                description: None,
                price: None,
                preview_image: None,
                discount: None,
            }
        )
    );
}
