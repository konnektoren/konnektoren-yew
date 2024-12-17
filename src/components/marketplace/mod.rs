mod cart_badge;
mod product;
mod product_catalog;
mod shopping_cart;
mod ton_wallet;

pub use cart_badge::CartBadgeComponent;
pub use product::ProductComponent;
pub use product_catalog::ProductCatalogComponent;
pub use shopping_cart::ShoppingCartComponent;
pub use ton_wallet::{payTonWallet, TonWalletComponent};
