use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod provider;
#[cfg(feature = "solana")]
pub mod solana;
pub mod token_selector;
pub mod ton;
pub mod wallet;
pub use provider::WalletProvider;
pub use wallet::{WalletComponent, WalletComponentProps};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Network {
    Ton,
    TonTestnet,
    Solana,
    SolanaDevnet,
    // Add other networks as needed
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Network::Ton => write!(f, "TON"),
            Network::TonTestnet => write!(f, "TON Testnet"),
            Network::Solana => write!(f, "Solana"),
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
