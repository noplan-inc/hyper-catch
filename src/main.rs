mod contract;
mod contract_getter;
mod formatter;
mod option;

use dotenv::dotenv;
use ethers_core::{abi::Abi, types::Address};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let rpc = env::var("EthMainRpc").expect("EthMainRpc must be set");
    let getter = contract_getter::NftContractGetter::new(&rpc);
    let hashmask = "0xC2C747E0F7004F9E8817Db2ca4997657a7746928".parse::<Address>()?;
    let uniswap = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse::<Address>()?;

    let contract_addresses = vec![hashmask, uniswap];

    let result = getter.extract_erc721_or_1155(contract_addresses).await;

    println!("{:?}", result);

    Ok(())
}
