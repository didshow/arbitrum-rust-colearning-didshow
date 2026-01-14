use alloy::{
    primitives::Address,
    providers::{ProviderBuilder},
    sol,
};
use eyre::Result;

// 1. 通过 sol! 宏导入本地 ABI 文件
sol!(
    #[sol(rpc)] // 生成 RPC 调用代码
    MyContract,
    "src/abi/erc20_abi.json"  
);

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // 4. 设置合约地址 
    let contract_address: Address = "0xdE066dceDD55fb16A0BE0491CC97bf0fF2354189".parse()?;

    // 5. 初始化合约实例
    let contract = MyContract::new(contract_address, provider);

    let name_response = contract.name().call().await?;
    println!("合约名称: {}", name_response._0); // _0 是 Alloy 对单返回值的默认命名

    let symbol_response = contract.symbol().call().await?;
    println!("合约符号: {}", symbol_response._0);

    Ok(())
}