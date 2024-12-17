use konnektoren_core::marketplace::Cart;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, Default)]
pub struct CartBadgeProps {
    pub cart: Cart,
    pub on_click: Callback<()>,
}

#[function_component(CartBadgeComponent)]
pub fn cart_badge(props: &CartBadgeProps) -> Html {
    let num_products = props.cart.products.len();
    let on_click = props.on_click.clone();

    let onclick = Callback::from(move |_| {
        on_click.emit(());
    });

    html! {
        <div class="cart__badge" {onclick}>
            <i class="cart__icon fas fa-shopping-cart"></i>
            <span class="cart__counter">{ num_products }</span>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::marketplace::{Cart, Product};
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        CartBadgeComponent,
        CartBadgeProps {
            on_click: Callback::noop(),
            cart: Cart {
                products: vec![
                    Product {
                        id: Some("1".to_string()),
                        name: "Test Product".to_string(),
                        price: Some(10.0),
                        ..Default::default()
                    },
                    Product {
                        id: Some("2".to_string()),
                        name: "Test Product 2".to_string(),
                        price: Some(20.0),
                        ..Default::default()
                    }
                ]
            },
            ..Default::default()
        },
        (
            "Empty cart",
            CartBadgeProps {
                on_click: Callback::noop(),
                cart: Cart { products: vec![] },
                ..Default::default()
            }
        )
    );
}
