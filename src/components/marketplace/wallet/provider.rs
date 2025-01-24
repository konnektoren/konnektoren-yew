use super::*;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait WalletProvider: Clone + PartialEq {
    async fn connect(&self) -> Result<WalletConnection, String>;
    async fn disconnect(&self) -> Result<WalletConnection, String>;
    async fn get_balance(&self, token: &Token) -> Result<WalletBalance, String>;
    async fn send_payment(
        &self,
        connection: &WalletConnection,
        recipient: &str,
        amount: u64,
        token: &Token,
    ) -> Result<String, String>;
    async fn fetch_token_price(&self, token: &Token) -> Result<f64, String>;
    fn is_connected(&self) -> bool;
    fn get_network(&self) -> Network;
    fn render_ui(
        &self,
        connection: Option<WalletConnection>,
        on_connect: Option<yew::Callback<WalletConnection>>,
        on_update: Option<yew::Callback<Self>>,
    ) -> yew::Html;
}
