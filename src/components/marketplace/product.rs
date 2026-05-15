use konnektoren_core::marketplace::Product;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct ProductComponentProps {
    pub product: Product,
    #[prop_or_default]
    pub is_highlighted: bool,
    #[prop_or_default]
    pub on_select: Option<Callback<Product>>,
    #[prop_or_default]
    pub on_tag: Option<Callback<String>>,
    #[prop_or("Get".to_string())]
    pub button_text: String,
}

#[function_component(ProductComponent)]
pub fn product_component(props: &ProductComponentProps) -> Html {
    let classes = classes!(
        "product",
        props.is_highlighted.then_some("product--highlighted")
    );

    let product_id = props.product.id.clone().unwrap_or_default();

    html! {
        <div id={format!("product-{}", product_id)} class={classes}>
            {render_body(props)}
        </div>
    }
}

fn render_header(product: &Product) -> Html {
    html! {
        <div class="product__header">
            <h2 class="product__title">
                {&product.name}
            </h2>
        </div>
    }
}

fn render_body(props: &ProductComponentProps) -> Html {
    html! {
        <div class="product-content">
            {render_header(&props.product)}
            {render_image(&props.product)}
            <p class="product__description">{&props.product.description}</p>
            {render_tags(props)}
            {render_footer(props)}
        </div>
    }
}

fn render_image(product: &Product) -> Html {
    match &product.image {
        Some(image) if image.starts_with("fa-") => html! {
            <div class="product__icon-container">
                <i class={format!("product__icon fas {}", image)}></i>
            </div>
        },
        Some(image) => html! {
            <figure>
                <img src={image.to_string()} alt={product.name.clone()} class="product__image" />
            </figure>
        },
        None => html!(),
    }
}

fn render_tags(props: &ProductComponentProps) -> Html {
    html! {
        <div class="product__tags">
            {props.product.tags.iter().map(|tag| render_tag(tag, props.on_tag.clone())).collect::<Html>()}
        </div>
    }
}

fn render_tag(tag: &str, on_tag: Option<Callback<String>>) -> Html {
    let tag_clone = tag.to_string();
    html! {
        <span
            class="product__tag"
            onclick={Callback::from(move |_| {
                if let Some(callback) = &on_tag {
                    callback.emit(tag_clone.clone());
                }
            })}
        >
            {tag}
        </span>
    }
}

fn render_footer(props: &ProductComponentProps) -> Html {
    html! {
        <div class="product__footer">
            {render_price(&props.product)}
            {render_button(props)}
        </div>
    }
}

fn render_price(product: &Product) -> Html {
    match product.price {
        Some(price) => html! {
            <div class="product__price">
                <span class="product__price-value">{format!("€{:.2}", price)}</span>
            </div>
        },
        None => html! {
            <div class="product__price">
                <span class="product__price-free">
                    <i class="fas fa-unlock"></i>
                    {" Free"}
                </span>
            </div>
        },
    }
}

fn render_button(props: &ProductComponentProps) -> Html {
    if let Some(on_select) = &props.on_select {
        let on_select = on_select.clone();
        let product = props.product.clone();
        let icon = if product.price.is_some() {
            "fa-cart-plus"
        } else {
            "fa-play"
        };
        html! {
            <button class="product__button" onclick={
                Callback::from(move |_| on_select.emit(product.clone()))
            }>
                <i class={format!("fas {}", icon)}></i>
                {" "}
                {&props.button_text}
            </button>
        }
    } else {
        html! {}
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ProductComponent,
        ProductComponentProps::default(),
        (
            "free with icon",
            ProductComponentProps {
                product: Product {
                    id: Some("test-1".to_string()),
                    name: "Artikel: der, die, das".to_string(),
                    description: "Lerne die Artikel für häufige Nomen.".to_string(),
                    price: None,
                    image: Some("fa-book-open".to_string()),
                    tags: vec!["c1".to_string(), "free".to_string()],
                    path: None
                },
                is_highlighted: false,
                on_select: Some(Callback::noop()),
                ..Default::default()
            }
        ),
        (
            "with price and icon",
            ProductComponentProps {
                product: Product {
                    id: Some("test-2".to_string()),
                    name: "Articles Mastery".to_string(),
                    description: "Dive deep into German articles with this interactive course. Learn to confidently use der, die, and das through pattern recognition.".to_string(),
                    price: Some(4.99),
                    image: Some("fa-graduation-cap".to_string()),
                    tags: vec!["c1".to_string(), "articles".to_string()],
                    path: None
                },
                is_highlighted: false,
                on_select: Some(Callback::noop()),
                ..Default::default()
            }
        ),
        (
            "highlighted",
            ProductComponentProps {
                product: Product {
                    id: Some("test-3".to_string()),
                    name: "Complete Article Package".to_string(),
                    description: "Master German articles with our comprehensive package. Includes structured lessons on der, die, and das.".to_string(),
                    price: Some(9.99),
                    image: Some("fa-palette".to_string()),
                    tags: vec!["c1".to_string(), "package".to_string(), "articles".to_string()],
                    path: None
                },
                is_highlighted: true,
                on_select: Some(Callback::noop()),
                ..Default::default()
            }
        )
    );
}
