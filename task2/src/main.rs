use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::Address;
use alloy_primitives::utils::format_units;
use std::error::Error;

#[tokio::main] 
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 设置 RPC URL
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    
    // 2. 创建 Provider
    let provider = ProviderBuilder::new().on_http(rpc_url); 

    // 3. 地址
    let wallet_address: Address = "0xC3229ca61461FfaED079c0B22dFb3B78A33c89B2".parse()?;

    // 4. 获取余额
    let balance_wei = provider.get_balance(wallet_address).await?;

    // 5. 转换单位
    let balance_eth = format_units(balance_wei, 18)?;

    println!("Wallet address: {wallet_address}");
    println!("Wallet balance (Wei): {balance_wei}");
    println!("Wallet balance in ETH: {balance_eth}");

    Ok(())
}