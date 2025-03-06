use super::*;
use async_trait::async_trait;
use gloo::net::http::Request;
use serde_json::json;
use solana_sdk::{hash::Hash, pubkey::Pubkey, system_instruction, transaction::Transaction};
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use wallet_adapter::{Cluster, SendOptions, Utils, WalletAdapter};
use yew::prelude::*;

#[derive(Clone)]
pub struct SolanaWalletProvider {
    adapter: Arc<RwLock<Option<WalletAdapter>>>,
    connection: Arc<RwLock<Option<WalletConnection>>>,
    network: Network,
}

impl PartialEq for SolanaWalletProvider {
    fn eq(&self, other: &Self) -> bool {
        // Compare network
        if self.network != other.network {
            return false;
        }

        // Compare connection states
        let connections_equal = match (self.connection.read(), other.connection.read()) {
            (Ok(self_conn), Ok(other_conn)) => *self_conn == *other_conn,
            _ => false,
        };

        if !connections_equal {
            return false;
        }

        // Compare adapter states (just check if they're both Some or both None)
        match (self.adapter.read(), other.adapter.read()) {
            (Ok(self_adapter), Ok(other_adapter)) => {
                self_adapter.is_some() == other_adapter.is_some()
            }
            _ => false,
        }
    }
}

impl SolanaWalletProvider {
    pub fn new(network: Network) -> Self {
        Self {
            adapter: Arc::new(RwLock::new(None)),
            connection: Arc::new(RwLock::new(None)),
            network,
        }
    }

    pub fn get_cluster(&self) -> Cluster {
        match self.get_network() {
            Network::SolanaDevnet => Cluster::DevNet,
            _ => Cluster::MainNet,
        }
    }

    fn create_wallet_button(
        &self,
        wallet_name: String,
        wallet_icon: Option<String>,
        on_connect: Option<Callback<WalletConnection>>,
        on_update: Option<Callback<Self>>,
    ) -> Html {
        let adapter_arc = self.adapter.clone();
        let connection = self.connection.clone();
        let wallet_name_display = wallet_name.clone();
        let network = self.get_network();

        let onclick = Callback::from(move |_| {
            let adapter_arc = adapter_arc.clone();
            let connection = connection.clone();
            let wallet_name = wallet_name.clone();
            let on_connect = on_connect.clone();
            let on_update = on_update.clone();
            let network = network.clone();

            wasm_bindgen_futures::spawn_local(async move {
                // Clear existing connection first
                if let Ok(mut conn_guard) = connection.write() {
                    *conn_guard = None;
                }

                if let Ok(mut adapter_guard) = adapter_arc.write() {
                    if adapter_guard.is_none() {
                        *adapter_guard = WalletAdapter::init().ok();
                    }

                    if let Some(ref mut adapter) = *adapter_guard {
                        log::info!("Attempting to connect to {}", wallet_name);

                        // Connect to the wallet
                        match adapter.connect_by_name(&wallet_name).await {
                            Ok(_) => {
                                gloo::timers::future::TimeoutFuture::new(500).await;

                                if let Ok(account) =
                                    adapter.connection_info().await.connected_account()
                                {
                                    log::info!("Got account: {}", account.address());
                                    let new_connection = WalletConnection {
                                        address: account.address().to_string(),
                                        network: network.clone(),
                                    };

                                    if let Ok(mut conn_guard) = connection.write() {
                                        *conn_guard = Some(new_connection.clone());
                                    }

                                    // Create updated provider instance
                                    let updated_provider = SolanaWalletProvider {
                                        adapter: adapter_arc.clone(),
                                        connection: connection.clone(),
                                        network,
                                    };

                                    // Emit the connection through the callbacks
                                    if let Some(cb) = on_connect {
                                        cb.emit(new_connection);
                                    }
                                    if let Some(cb) = on_update {
                                        cb.emit(updated_provider);
                                    }
                                }
                            }
                            Err(e) => log::error!("Connection failed: {}", e),
                        }
                    }
                }
            });
        });

        html! {
            <button class="wallet__connect-button" onclick={onclick}>
                if let Some(icon) = wallet_icon {
                    <img src={icon} alt={format!("{} icon", wallet_name_display)} />
                }
                {wallet_name_display}
            </button>
        }
    }

    fn has_adapter(&self) -> bool {
        self.adapter
            .read()
            .map(|guard| guard.is_some())
            .unwrap_or(false)
    }

    fn get_adapter_wallets(&self) -> Vec<wallet_adapter::Wallet> {
        if let Ok(guard) = self.adapter.read() {
            if let Some(adapter) = guard.as_ref() {
                return adapter.wallets();
            }
        }
        vec![]
    }

    fn render_wallet_list(
        &self,
        on_connect: Option<Callback<WalletConnection>>,
        on_update: Option<Callback<Self>>,
    ) -> Html {
        html! {
            <div class="wallet__connect-list">
                {
                    self.get_adapter_wallets().iter().map(|wallet| {
                        let wallet_name = wallet.name().to_string();
                        let wallet_icon = wallet.icon().map(|i| i.to_string());
                        self.create_wallet_button(
                            wallet_name,
                            wallet_icon,
                            on_connect.clone(),
                            on_update.clone()
                        )
                    }).collect::<Html>()
                }
            </div>
        }
    }

    fn render_ui_content(
        &self,
        on_connect: Option<Callback<WalletConnection>>,
        on_update: Option<Callback<Self>>,
    ) -> Html {
        if !self.has_adapter() {
            self.render_initialize_button(on_update)
        } else {
            let wallets = self.get_adapter_wallets();
            if wallets.is_empty() {
                self.render_loading()
            } else {
                self.render_wallet_list(on_connect, on_update)
            }
        }
    }

    fn render_initialize_button(&self, on_update: Option<Callback<Self>>) -> Html {
        let adapter = self.adapter.clone();
        let network = self.get_network();

        let onclick = Callback::from(move |_| {
            let adapter = adapter.clone();
            let on_update = on_update.clone();
            let network = network.clone();

            wasm_bindgen_futures::spawn_local(async move {
                // Initialize adapter
                if let Ok(mut adapter_guard) = adapter.write() {
                    if let Ok(new_adapter) = WalletAdapter::init() {
                        let _ = new_adapter.devnet();
                        *adapter_guard = Some(new_adapter);

                        // Create new provider instance with updated adapter
                        let updated_provider = SolanaWalletProvider {
                            adapter: adapter.clone(),
                            connection: Arc::new(RwLock::new(None)),
                            network,
                        };

                        // Emit the updated provider through the callback
                        if let Some(cb) = on_update {
                            cb.emit(updated_provider);
                        }
                    }
                }
            });
        });

        html! {
            <div class="wallet__connect-wrapper">
                <button class="wallet__connect-button" onclick={onclick}>
                    <i class="fas fa-power-off" />
                    {"Initialize Solana Wallet"}
                </button>
            </div>
        }
    }

    fn render_loading(&self) -> Html {
        html! {
            <div class="wallet__loading">
                <div class="spinner"></div>
                {"Initializing Solana wallet..."}
            </div>
        }
    }
}

#[async_trait(?Send)]
impl WalletProvider for SolanaWalletProvider {
    async fn connect(&self) -> Result<WalletConnection, String> {
        // Check if already connected
        if let Ok(connection_guard) = self.connection.read() {
            if let Some(existing) = connection_guard.as_ref() {
                return Ok(existing.clone());
            }
        }

        // Initialize new adapter if none exists
        let new_adapter = WalletAdapter::init()
            .ok()
            .ok_or("Failed to initialize adapter")?;

        // Store the new adapter
        if let Ok(mut adapter_guard) = self.adapter.write() {
            *adapter_guard = Some(new_adapter);
        } else {
            return Err("Failed to store adapter".to_string());
        }

        // Get the adapter reference
        let adapter_guard = self.adapter.read().map_err(|_| "Failed to read adapter")?;
        let adapter = adapter_guard.as_ref().ok_or("No adapter available")?;

        // Check if wallet is connected
        if !adapter.is_connected().await {
            return Err(
                "Wallet not connected. Please connect using the wallet list first.".to_string(),
            );
        }

        // Get connected account
        let connect_info = adapter.connection_info().await;
        let account = connect_info
            .connected_account()
            .map_err(|e| e.to_string())?;

        // Create connection
        let connection = WalletConnection {
            address: account.address().to_string(),
            network: self.get_network(),
        };

        // Store connection
        if let Ok(mut conn_guard) = self.connection.write() {
            *conn_guard = Some(connection.clone());
        }

        // Wait a moment to stabilize
        gloo::timers::future::TimeoutFuture::new(200).await;

        Ok(connection)
    }

    async fn disconnect(&self) -> Result<WalletConnection, String> {
        // Acquire write lock on adapter
        let mut adapter_guard = self
            .adapter
            .write()
            .map_err(|_| "Failed to acquire adapter write lock".to_string())?;

        // Acquire write lock on connection
        let mut conn_guard = self
            .connection
            .write()
            .map_err(|_| "Failed to acquire connection write lock".to_string())?;

        // Take the current connection
        let old_connection = conn_guard.take();

        if let (Some(adapter), Some(connection)) = (&mut *adapter_guard, old_connection.clone()) {
            // Perform disconnect
            adapter.disconnect().await;

            // Wait a moment for the connection to stabilize
            gloo::timers::future::TimeoutFuture::new(200).await;

            // Clear adapter state
            *adapter_guard = None;

            Ok(connection)
        } else {
            Err("No adapter available or no connection to disconnect".to_string())
        }
    }

    async fn get_balance(&self, token: &Token) -> Result<WalletBalance, String> {
        if let Ok(connection_guard) = self.connection.read() {
            if let Some(connection) = &*connection_guard {
                match token.token_type {
                    TokenType::Native => {
                        let balance =
                            get_sol_balance(&connection.address, &self.get_cluster()).await?;
                        Ok(WalletBalance {
                            amount: balance as f64 / 1_000_000_000.0,
                            token: token.clone(),
                        })
                    }
                    TokenType::Custom => {
                        let balance = get_token_balance(
                            &connection.address,
                            &token.contract_address,
                            &self.get_cluster(),
                        )
                        .await?;
                        Ok(WalletBalance {
                            amount: balance as f64 / 10f64.powi(token.decimals as i32),
                            token: token.clone(),
                        })
                    }
                }
            } else {
                Err("Wallet not connected".to_string())
            }
        } else {
            Err("Failed to acquire connection lock".to_string())
        }
    }

    async fn send_payment(
        &self,
        _connection: &WalletConnection,
        recipient: &str,
        amount: u64,
        token: &Token,
    ) -> Result<String, String> {
        // Get adapter read lock
        let adapter_guard = self
            .adapter
            .read()
            .map_err(|_| "Failed to acquire adapter read lock".to_string())?;

        // Get adapter reference
        let adapter = adapter_guard
            .as_ref()
            .ok_or_else(|| "No adapter available".to_string())?;

        // Get connection read lock
        let connection_guard = self
            .connection
            .read()
            .map_err(|_| "Failed to acquire connection read lock".to_string())?;

        // Check both adapter connection and our connection state
        if !adapter.is_connected().await || connection_guard.is_none() {
            return Err("Wallet not connected".to_string());
        }

        // Proceed with transaction
        let signature = self
            .send_solana_transaction(recipient, amount, token, adapter)
            .await?;
        Ok(signature)
    }

    fn is_connected(&self) -> bool {
        self.connection
            .read()
            .map(|conn| conn.is_some())
            .unwrap_or(false)
    }

    fn get_network(&self) -> Network {
        self.network.clone()
    }

    fn render_ui(
        &self,
        _connection: Option<WalletConnection>,
        on_connect: Option<Callback<WalletConnection>>,
        on_update: Option<Callback<Self>>,
    ) -> Html {
        self.render_ui_content(on_connect, on_update)
    }

    async fn fetch_token_price(&self, token: &Token) -> Result<f64, String> {
        let url = match token.token_type {
            TokenType::Native => {
                "https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd"
            }
            TokenType::Custom => {
                return Err("Price fetching for custom tokens not implemented".to_string());
            }
        };

        let response = gloo::net::http::Request::get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

        // Parse the response according to CoinGecko API structure
        json["solana"]["usd"]
            .as_f64()
            .ok_or("Failed to parse SOL price".to_string())
    }
}

impl SolanaWalletProvider {
    async fn get_blockhash(&self) -> Result<Hash, String> {
        // Use gloo-net for making the request
        let cluster = self.get_cluster();
        let endpoint = cluster.endpoint();
        let body = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "getLatestBlockhash",
            "params": []
        });

        let response = Request::post(endpoint)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .unwrap()
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

        let blockhash = json["result"]["value"]["blockhash"]
            .as_str()
            .ok_or("Failed to parse blockhash".to_string())?;

        Hash::from_str(blockhash).map_err(|e| e.to_string())
    }

    fn build_transaction(
        &self,
        recipient: &str,
        amount: u64,
        token: &Token,
        from_pubkey: &Pubkey,
        recent_blockhash: &Hash,
    ) -> Result<Transaction, String> {
        // Verify the recipient address is valid
        let recipient_pubkey = Pubkey::from_str(recipient).map_err(|_| {
            log::error!("Invalid recipient address: {}", recipient);
            "Invalid recipient address".to_string()
        })?;
        log::info!("Recipient pubkey: {}", recipient_pubkey);

        // Build the transaction
        let mut tx = match token.token_type {
            TokenType::Native => {
                log::info!("Creating native SOL transfer for {} lamports", amount);
                let transfer_instruction =
                    system_instruction::transfer(from_pubkey, &recipient_pubkey, amount);
                Transaction::new_with_payer(&[transfer_instruction], Some(from_pubkey))
            }
            TokenType::Custom => {
                let token_program_id =
                    Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
                        .map_err(|e| e.to_string())?;
                let token_mint =
                    Pubkey::from_str(&token.contract_address).map_err(|e| e.to_string())?;
                let amount_with_decimals = amount
                    .checked_mul(10u64.pow(token.decimals as u32))
                    .ok_or_else(|| "Amount overflow".to_string())?;

                log::info!(
                    "Creating SPL token transfer for {} tokens (decimals: {})",
                    amount_with_decimals,
                    token.decimals
                );
                let transfer_instruction = spl_token::instruction::transfer_checked(
                    &token_program_id,
                    from_pubkey,
                    &token_mint,
                    &recipient_pubkey,
                    from_pubkey,
                    &[from_pubkey],
                    amount_with_decimals,
                    token.decimals,
                )
                .map_err(|e| {
                    log::error!("Error creating SPL transfer instruction: {}", e);
                    e.to_string()
                })?;

                Transaction::new_with_payer(&[transfer_instruction], Some(from_pubkey))
            }
        };
        tx.message.recent_blockhash = *recent_blockhash;
        Ok(tx)
    }

    async fn sign_and_send_transaction(
        &self,
        tx: Transaction,
        adapter: &WalletAdapter,
    ) -> Result<String, String> {
        // Serialize the transaction
        let tx_bytes = bincode::serialize(&tx).map_err(|e| {
            log::error!("Failed to serialize transaction: {}", e);
            format!("Failed to prepare transaction: {}", e)
        })?;

        log::info!("Sending transaction...");

        // Send the transaction
        match adapter
            .sign_and_send_transaction(&tx_bytes, self.get_cluster(), SendOptions::default())
            .await
        {
            Ok(signature) => {
                let signature = Utils::base58_signature(signature);
                log::info!("Transaction sent successfully: {}", signature);
                Ok(signature)
            }
            Err(e) => {
                log::error!("Transaction failed: {}", e);
                Err(format!("Transaction failed: {}", e))
            }
        }
    }
    async fn send_solana_transaction(
        &self,
        recipient: &str,
        amount: u64,
        token: &Token,
        adapter: &WalletAdapter,
    ) -> Result<String, String> {
        let connect_info = adapter.connection_info().await;
        let account = connect_info
            .connected_account()
            .map_err(|e| e.to_string())?;
        let from_pubkey = Pubkey::from(account.public_key());
        log::info!("From pubkey: {}", from_pubkey);

        let recent_blockhash = self.get_blockhash().await?;

        let tx =
            self.build_transaction(recipient, amount, token, &from_pubkey, &recent_blockhash)?;
        let signature = self.sign_and_send_transaction(tx, adapter).await?;
        Ok(signature)
    }
}

// Helper functions for balance queries
async fn get_sol_balance(address: &str, cluster: &Cluster) -> Result<u64, String> {
    use serde_json::json;

    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBalance",
        "params": [address]
    });

    let response = gloo::net::http::Request::post(cluster.endpoint())
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .unwrap()
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    #[derive(serde::Deserialize)]
    struct BalanceResponse {
        result: Balance,
    }

    #[derive(serde::Deserialize)]
    struct Balance {
        value: u64,
    }

    let balance_response: BalanceResponse =
        serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(balance_response.result.value)
}

async fn get_token_balance(
    wallet_address: &str,
    token_address: &str,
    cluster: &Cluster,
) -> Result<u64, String> {
    use gloo::net::http::Request;
    use serde_json::json;

    let endpoint = cluster.endpoint();
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTokenAccountsByOwner",
        "params": [
            wallet_address,
            {
                "mint": token_address
            },
            {
                "encoding": "jsonParsed"
            }
        ]
    });

    let resp = Request::post(endpoint)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .unwrap()
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = resp.text().await.map_err(|e| e.to_string())?;

    #[derive(serde::Deserialize)]
    struct TokenAccountResponse {
        result: TokenAccounts,
    }

    #[derive(serde::Deserialize)]
    struct TokenAccounts {
        value: Vec<TokenAccount>,
    }

    #[derive(serde::Deserialize)]
    struct TokenAccount {
        account: AccountInfo,
    }

    #[derive(serde::Deserialize)]
    struct AccountInfo {
        data: TokenAccountData,
    }

    #[derive(serde::Deserialize)]
    struct TokenAccountData {
        parsed: ParsedAccountInfo,
    }

    #[derive(serde::Deserialize)]
    struct ParsedAccountInfo {
        info: TokenAccountInfo,
    }

    #[derive(serde::Deserialize)]
    struct TokenAccountInfo {
        #[serde(rename = "tokenAmount")]
        token_amount: TokenAmount,
    }

    #[derive(serde::Deserialize)]
    struct TokenAmount {
        amount: String,
    }

    let response: TokenAccountResponse =
        serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {}", e))?;

    if let Some(account) = response.result.value.first() {
        let amount = account
            .account
            .data
            .parsed
            .info
            .token_amount
            .amount
            .parse::<u64>()
            .map_err(|e| format!("Failed to parse amount: {}", e))?;
        Ok(amount)
    } else {
        Ok(0)
    }
}
