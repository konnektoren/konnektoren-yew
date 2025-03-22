use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod provider;
#[cfg(feature = "solana")]
pub mod solana;
pub mod token_selector;
#[cfg(feature = "csr")]
pub mod ton;
pub mod wallet;
pub use provider::WalletProvider;
pub use wallet::{WalletComponent, WalletComponentProps};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Network {
    #[cfg(feature = "csr")]
    Ton,
    #[cfg(feature = "csr")]
    TonTestnet,
    #[cfg(feature = "solana")]
    Solana,
    #[cfg(feature = "solana")]
    SolanaDevnet,
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "csr")]
            Network::Ton => write!(f, "TON"),
            #[cfg(feature = "csr")]
            Network::TonTestnet => write!(f, "TON Testnet"),
            #[cfg(feature = "solana")]
            Network::Solana => write!(f, "Solana"),
            #[cfg(feature = "solana")]
            Network::SolanaDevnet => write!(f, "Solana Devnet"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    Native,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub image_url: String,
    pub contract_address: String,
    pub decimals: u8,
    pub network: Network,
    pub token_type: TokenType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WalletBalance {
    pub amount: f64,
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WalletConnection {
    pub address: String,
    pub network: Network,
}
