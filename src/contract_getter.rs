use ethers_contract::Contract;
use std::{fs, io};

use ethers::{abi::Abi, prelude::*};
use ethers_providers::{Http, Middleware, Provider};

const ERC721_INTERFACE_ID: [u8; 4] = [128, 172, 88, 205];
const ERC1155_INTERFACE_ID: [u8; 4] = [217, 182, 122, 38];

pub struct NftContractGetter {
    pub provider: Provider<Http>,
}

impl NftContractGetter {
    pub fn new(rpc_end_point: &str) -> NftContractGetter {
        NftContractGetter {
            provider: Provider::<Http>::try_from(rpc_end_point).unwrap(),
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
    pub async fn _is_erc721(&self, contract_address: H160) -> Result<bool, io::Error> {
        let erc721_abi: Abi = serde_json::from_str(&(fs::read_to_string("src/abis/erc721.json")?))?;
        let contract = Contract::new(contract_address, erc721_abi, &self.provider);

        let support_interface_result = contract
            .method::<_, bool>("supportsInterface", ERC721_INTERFACE_ID)
            .unwrap()
            .call()
            .await;

        match support_interface_result {
            Ok(is_erc721) => {
                return Ok(is_erc721);
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    "supportInterface doesn't exist",
                ));
            }
        }
    }
    pub async fn is_erc1155(&self, contract_address: H160) -> Result<bool, io::Error> {
        let erc1155_abi: Abi =
            serde_json::from_str(&(fs::read_to_string("src/abis/erc1155.json")?))?;
        let contract = Contract::new(contract_address, erc1155_abi, &self.provider);

        let support_interface_result = contract
            .method::<_, bool>("supportsInterface", ERC1155_INTERFACE_ID)
            .unwrap()
            .call()
            .await;

        match support_interface_result {
            Ok(is_erc1155) => {
                return Ok(is_erc1155);
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    "supportInterface doesn't exist",
                ));
            }
        }
    }
    pub async fn is_erc721(
        &self,
        contract_address: H160,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let erc721_abi: Abi = serde_json::from_str(&(fs::read_to_string("src/abis/erc721.json")?))?;
        let contract = Contract::new(contract_address, erc721_abi, &self.provider);

        let support_interface_result = contract
            .method::<_, bool>("supportsInterface", ERC721_INTERFACE_ID)?
            .call()
            .await?;

        Ok(support_interface_result)
    }
    pub async fn extract_erc721_or_1155(&self, contracts_addresses: Vec<H160>) -> Vec<(H160, u8)> {
        let mut nft_contracts: Vec<(H160, u8)> = Vec::new();

        for contract_address in contracts_addresses {
            if self.is_erc721(contract_address).await.unwrap_or(false) {
                nft_contracts.push((contract_address, 0));
                continue;
            }

            if self.is_erc1155(contract_address).await.unwrap_or(false) {
                nft_contracts.push((contract_address, 1));
            }
        }
        nft_contracts
    }
}
