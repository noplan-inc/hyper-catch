use ethers::{abi::ethereum_types, prelude::*};
use ethers_providers::{Http, Middleware, Provider};

pub struct NftContractGetter {
    pub provider: Provider<Http>,
}

impl NftContractGetter {
    pub fn new(rpcEndPoint: &str) -> NftContractGetter {
        NftContractGetter {
            provider: Provider::<Http>::try_from(rpcEndPoint).unwrap(),
        }
    }

    pub async fn get_contract_creation_tx_hashes(&self, block_number: u64) -> Vec<H256> {
        let block = self
            .provider
            .get_block_with_txs(block_number)
            .await
            .unwrap()
            .unwrap();
        let mut contract_creation_tx_hashes: Vec<H256> = Vec::new();
        for tx in block.transactions {
            match tx.to {
                None => contract_creation_tx_hashes.push(tx.hash),
                _ => {}
            }
        }
        contract_creation_tx_hashes
    }
    pub async fn get_contracts_from_creations(
        &self,
        contract_creation_tx_hashes: Vec<H256>,
    ) -> Vec<H160> {
        let mut contract_addresses: Vec<H160> = Vec::new();
        for tx_hash in contract_creation_tx_hashes {
            let receipt = self
                .provider
                .get_transaction_receipt(tx_hash)
                .await
                .unwrap()
                .unwrap();

            contract_addresses.push(receipt.contract_address.unwrap());
        }
        contract_addresses
    }
}
