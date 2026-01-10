use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::Address;
use std::error::Error;
use alloy::sol;

sol! { 
   #[sol(rpc)] 
   contract HelloWeb3 { 
        function hello_web3() pure public returns(string memory); 
   } 
} 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to Arbitrum Sepolia testnet
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url); 
    
    // get latest block number
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {latest_block}");

    // task 1: print "Hello web3" to the console
    println!("Hello web3");
    
    let contract_address: Address = "0x2ab59788a255155CE1160C2549463de13D9Fa2b6".parse()?;
    let contract = HelloWeb3::new(contract_address, provider);
    let message = contract.hello_web3().call().await?;

    println!("Contract message: {}", message);

    Ok(())
}