use super::product::ProductComponent;
use konnektoren_core::marketplace::{Cart, Product};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ShoppingCartProps {
    pub cart: Cart,
    #[prop_or_default]
    pub on_select: Option<Callback<Product>>,
}

#[function_component(ShoppingCartComponent)]
pub fn shopping_cart_component(props: &ShoppingCartProps) -> Html {
    let button_text = "Remove".to_string();
    let on_select = props.on_select.clone();
    let products = props
        .cart
        .products
        .iter()
        .map(|product| {
            html! {
                <ProductComponent product={product.clone()} on_select={on_select.clone()} button_text={button_text.clone()} />
            }
        })
        .collect::<Html>();

    html! {
        <div class="shopping-cart">
        { products }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::marketplace::{Cart, Product};
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ShoppingCartComponent,
        ShoppingCartProps {
            cart: Cart {
                products: vec![],
                ..Default::default()
            },
            on_select: None,
        },
        (
            "with products",
            ShoppingCartProps {
                cart: Cart {
                    products: vec![
                        Product {
                            id: None,
                            name: "Test Product".to_string(),
                            description: "This is a Test Product".to_string(),
                            ..Default::default()
                        },
                        Product {
                            id: None,
                            name: "Test Product 2".to_string(),
                            description: "This is a Test Product 2".to_string(),
                            price: Some(10.0),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                },
                on_select: None,
            }
        )
    );
}
