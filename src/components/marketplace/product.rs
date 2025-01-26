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
            {render_header(&props.product)}
            {render_body(props)}
            {render_footer(props)}
        </div>
    }
}

fn render_header(product: &Product) -> Html {
    html! {
        <div class="product__header">
            <h2 class="product__title">{&product.name}</h2>
        </div>
    }
}

fn render_body(props: &ProductComponentProps) -> Html {
    html! {
        <div class="product__body">
            <p class="product__description">{&props.product.description}</p>
            {render_image(&props.product)}
            {render_tags(props)}
        </div>
    }
}

fn render_image(product: &Product) -> Html {
    match &product.image {
        Some(image) if image.starts_with("fa-") => html! {
            <i class={format!("product__icon fas {}", image)}></i>
        },
        Some(image) => html! {
            <img src={image.to_string()} alt={product.name.clone()} class="product__image" />
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
            <span class="product__price">{format!("Price: {}", price)}</span>
        },
        None => html!(),
    }
}

fn render_button(props: &ProductComponentProps) -> Html {
    if let Some(on_select) = &props.on_select {
        let on_select = on_select.clone();
        let product = props.product.clone();
        html! {
            <button class="product__button" onclick={
                Callback::from(move |_| on_select.emit(product.clone()))
            }>
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
            "no price",
            ProductComponentProps {
                product: Product {
                    id: Some("test-1".to_string()),
                    name: "Test Product".to_string(),
                    description: "This is a Test Product".to_string(),
                    price: None,
                    image: None,
                    tags: vec![],
                    path: None
                },
                is_highlighted: false,
                on_select: None,
                ..Default::default()
            }
        ),
        (
            "with price",
            ProductComponentProps {
                product: Product {
                    id: Some("test-2".to_string()),
                    name: "Test Product".to_string(),
                    description: "This is a Test Product".to_string(),
                    price: Some(1.0),
                    image: None,
                    tags: vec!["tag1".to_string(), "tag2".to_string()],
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
                    name: "Highlighted Product".to_string(),
                    description: "This is a highlighted product".to_string(),
                    price: Some(1.0),
                    image: None,
                    tags: vec!["highlighted".to_string()],
                    path: None
                },
                is_highlighted: true,
                on_select: Some(Callback::noop()),
                ..Default::default()
            }
        )
    );
}
