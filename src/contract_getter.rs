use crate::contract;
use crate::nft_types;
use crate::nft_types::NftTypes;
use crate::safe_ethers;

use anyhow::Result;
use ethers::types;

pub struct NftContractGetter {
    pub safe_ethers: safe_ethers::SafeEthers,
}

impl NftContractGetter {
    pub async fn new(rpc_end_point: &str, chain_name: String) -> Result<NftContractGetter> {
        Ok(NftContractGetter {
            safe_ethers: safe_ethers::SafeEthers::new(rpc_end_point, chain_name).await?,
        })
    }

    async fn get_contract_creation_tx_hashes(&mut self, block_number: u64) -> Vec<types::H256> {
        //create array of transaction hashes, on which a contract is deployed.
        let mut contract_creation_tx_hashes: Vec<types::H256> = Vec::new();

        let get_block_with_txs_option =
            self.safe_ethers.safe_get_block_with_txs(block_number).await;

        if let Some(block_with_txs) = get_block_with_txs_option {
            for tx in block_with_txs.transactions {
                if tx.to == None {
                    contract_creation_tx_hashes.push(tx.hash)
                }
            }
        }
        contract_creation_tx_hashes
    }

    async fn get_contracts_from_creations(
        &mut self,
        contract_creation_tx_hashes: Vec<types::H256>,
    ) -> Vec<types::H160> {
        let mut contract_addresses: Vec<types::H160> = Vec::new();

        for tx_hash in contract_creation_tx_hashes {
            let get_transaction_receipt_option =
                self.safe_ethers.safe_get_transaction_receipt(tx_hash).await;

            if let Some(receipt) = get_transaction_receipt_option {
                if let Some(contract_address) = receipt.contract_address {
                    contract_addresses.push(contract_address);
                } else {
                    tracing::error!(
                        "{} : Unexpected error empty contract_address tx_hash : {}",
                        self.safe_ethers.chain_name,
                        tx_hash,
                    );
                }
            }
        }

        contract_addresses
    }

    pub async fn extract_nft_contract_addresses(
        &mut self,
        contracts_addresses: Vec<types::H160>,
    ) -> Vec<(ethers::types::H160, nft_types::NftTypes)> {
        let mut nft_contracts: Vec<(types::H160, nft_types::NftTypes)> = Vec::new();

        for contract_address in contracts_addresses {
            match self
                .safe_ethers
                .is_nft_of(nft_types::NftTypes::Erc721, contract_address)
                .await
            {
                Ok(o) if o => {
                    nft_contracts.push((contract_address, nft_types::NftTypes::Erc721));
                    continue;
                }
                //when supportInterface returns false
                Ok(_) => (),
                //when contract doesn't have supportsInterface
                Err(_) => continue,
            }

            match self
                .safe_ethers
                .is_nft_of(nft_types::NftTypes::Erc1155, contract_address)
                .await
            {
                Ok(o) if o => {
                    nft_contracts.push((contract_address, nft_types::NftTypes::Erc1155));
                    continue;
                }
                _ => (),
            }
        }
        nft_contracts
    }

    pub async fn find(&mut self, block_number: u64) -> Vec<contract::Contract> {
        let contract_creation_tx_hashes = self.get_contract_creation_tx_hashes(block_number).await;

        let deployed_contract_addresses = self
            .get_contracts_from_creations(contract_creation_tx_hashes)
            .await;

        let nft_contract_addresses = self
            .extract_nft_contract_addresses(deployed_contract_addresses)
            .await;

        let formatted_nft_contracts = nft_contract_addresses
            .iter()
            .map(|contract| match contract.1 {
                NftTypes::Erc721 => contract::Contract {
                    block_number,
                    address: format!("0x{:x}", contract.0),
                    interface_ids: contract::InterfaceIds(
                        vec![NftTypes::Erc721.interface_id_str()],
                    ),
                },
                NftTypes::Erc1155 => contract::Contract {
                    block_number,
                    address: format!("0x{:x}", contract.0),
                    interface_ids: contract::InterfaceIds(vec![
                        NftTypes::Erc1155.interface_id_str()
                    ]),
                },
            })
            .collect::<Vec<_>>();

        formatted_nft_contracts
    }
}
