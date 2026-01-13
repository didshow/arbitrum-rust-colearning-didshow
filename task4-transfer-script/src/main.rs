use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{utils::parse_ether, Address},
    providers::{Provider, ProviderBuilder,WalletProvider},
    signers::local::PrivateKeySigner,
    rpc::types::eth::TransactionRequest,
};
use dotenvy::dotenv;
use eyre::Result;
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 加载环境变量并初始化
    dotenv().ok();
    let rpc_url = env::var("RPC_URL")?.parse()?;
    let private_key = env::var("PRIVATE_KEY")?;
    let to_addr_str = env::var("TO_ADDRESS")?;

    // 2. 配置签名者与 Provider
    let signer: PrivateKeySigner = private_key.parse()?;
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new()
        .with_recommended_fillers() 
        .wallet(wallet)
        .on_http(rpc_url);

    // 3. 地址校验 (Arbitrum Sepolia 地址格式)
    let from_addr = provider.default_signer_address();
    let to_addr = Address::from_str(&to_addr_str)
        .map_err(|_| eyre::eyre!("invalid to_address"))?;

    println!("Transfering...");
    println!("From: {:?}", from_addr);
    println!("To:   {:?}", to_addr);

    // 4. 检查余额
    let balance = provider.get_balance(from_addr).await?;
    println!("Current Balance: {} ETH", alloy::primitives::utils::format_ether(balance));

    // 5. 构建交易请求  0.0001ETH
    let amount = parse_ether("0.0001")?;
    
    let tx = TransactionRequest::default()
        .with_to(to_addr)
        .with_value(amount);

    // 6. 发送交易并等待确认
    // alloy 的 filler 会自动处理关卡 3 中的 Gas 估算与 Nonce 填充
    println!("estimate Gas and send transaction...");
    let pending_tx = provider.send_transaction(tx).await?;
    
    let tx_hash = pending_tx.inner().tx_hash();
    println!("Transaction Commited! Hash: {:?}", tx_hash);

    // 7. 等待交易回执 (确认成功)
    let receipt = pending_tx.get_receipt().await?;
    println!("Successfully! Block Height: {:?}", receipt.block_number);

    Ok(())
}