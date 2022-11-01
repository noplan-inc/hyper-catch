use hex;

mod contract;
mod contract_getter;
mod formatter;
mod option;

use ethers_contract::Contract;

use dotenv::dotenv;
use ethers_core::{abi::Abi, types::Address};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let rpc = env::var("EthMainRpc").expect("EthMainRpc must be set");
    let getter = contract_getter::NftContractGetter::new(&rpc);
    // this is a fake address used just for this example
    let address = "0xC2C747E0F7004F9E8817Db2ca4997657a7746928".parse::<Address>()?;

    // (ugly way to write the ABI inline, you can otherwise read it from a file)
    let abi: Abi = serde_json::from_str(
        r#"[{
            "inputs": [
              {
                "internalType": "bytes4",
                "name": "interfaceId",
                "type": "bytes4"
              }
            ],
            "name": "supportsInterface",
            "outputs": [
              {
                "internalType": "bool",
                "name": "",
                "type": "bool"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          },
          {
            "constant": true,
            "inputs": [],
            "name": "name",
            "outputs": [
              {
                "name": "",
                "type": "string"
              }
            ],
            "payable": false,
            "stateMutability": "view",
            "type": "function"
          }]"#,
    )?;

    // connect to the network

    // create the contract object at the address
    let contract = Contract::new(address, abi, getter.provider);

    // let init_value: String = contract.method::<_, String>("name", ())?.call().await?;
    let init_value = contract
        .method::<_, bool>("supportsInterface", (hex::decode("80ac58cd").unwrap(),))?
        .call()
        .await?;
    println!("{:?}", init_value);

    Ok(())
}

// let hashes = getter.get_contract_creation_tx_hashes(3000000).await;

//     let contracts = getter.get_contracts_from_creations(hashes).await;
//     println!("{:?}", contracts);
