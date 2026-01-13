use alloy::{
    network::Ethereum,
    primitives::{Address, U256, utils::format_units},
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::TransactionRequest,
    transports::Transport,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 设置 Arbitrum Sepolia RPC 节点地址
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url); 

    let from_addr: Address = "0x3C2e8188f4F000aEF8E85a2Fa1cDd69224c92715".parse()?;
    let to_addr: Address = "0x71fAdFD0afa5782CCf83fC0398230287788bF6bD".parse()?;
    
    let tx = TransactionRequest::default()
        .from(from_addr)
        .to(to_addr)
        .value(U256::from(100000000000000u64)); // 0.0001 ETH

    // 3. 执行获取与计算
    let gas_info = calculate_transfer_gas_fee(&provider, tx).await?;

    // 4. 打印结果
    println!("--- Arbitrum Sepolia Gas Report ---");
    println!("Gas Price: {} Gwei", (gas_info.gas_price as f64 / 1e9));
    println!("Gas Limit: {}", gas_info.gas_limit);
    println!("Total Gas Fee: {} Wei", gas_info.total_fee);
    println!("Total Gas Fee: {} ETH", format_units(gas_info.total_fee, 18)?);
    Ok(())
}

struct GasEstimate {
    gas_price: u128,
    gas_limit: u128,
    total_fee: U256,
}

/// 动态获取 Gas 价格与限额并计算
async fn calculate_transfer_gas_fee<T, P>(
    provider: &P,
    tx: TransactionRequest,
) -> Result<GasEstimate>
where
    T: Transport + Clone,
    P: Provider<T, Ethereum>, {
    // 动态获取实时 Gas 价格 (get_gas_price)
    let gas_price = provider.get_gas_price().await?;

    // 动态估算 Gas 限额 (eth_estimateGas)
    let gas_limit = provider.estimate_gas(&tx).await?;

    // Gas Fee = Gas Price × Gas Limit
    let total_fee = U256::from(gas_price) * U256::from(gas_limit);

    Ok(GasEstimate {
        gas_price,
        gas_limit,
        total_fee,
    })
}
