use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Check if a wallet is a whale (holds minimum SOL balance)
pub fn is_whale(
    rpc_client: &RpcClient,
    wallet_address: &str,
    minimum_sol: f64,
) -> Result<bool> {
    let sol_balance = get_sol_balance(rpc_client, wallet_address)?;
    Ok(sol_balance >= minimum_sol)
}

/// Get the SOL balance of a wallet
pub fn get_sol_balance(
    rpc_client: &RpcClient,
    wallet_address: &str,
) -> Result<f64> {
    let pubkey = Pubkey::from_str(wallet_address)
        .map_err(|e| anyhow!("Invalid wallet address: {}", e))?;
    
    let lamports = rpc_client
        .get_balance(&pubkey)
        .map_err(|e| anyhow!("Failed to fetch balance: {}", e))?;
    
    // Convert lamports to SOL (1 SOL = 1_000_000_000 lamports)
    Ok(lamports as f64 / 1_000_000_000.0)
}

/// Format SOL balance for display
pub fn format_sol_balance(sol: f64) -> String {
    if sol >= 1000.0 {
        format!("{:.2}K SOL", sol / 1000.0)
    } else {
        format!("{:.2} SOL", sol)
    }
}