//ethersのメソッドをエラーが発生したらpanicさせずにlogに書き込むようにして扱いやすくするためのもの
use crate::nft_types;
use anyhow::{anyhow, Result};
use std::fs;

use ethers_contract::AbiError;
use ethers_contract::ContractError;
use ethers_core::abi;
use ethers_core::abi::Error;
use ethers_core::types;
use ethers_core::types::transaction::response;
use ethers_providers::{Middleware, ProviderError};

pub struct SafeEthers {
    pub chain_name: String,
    pub provider: ethers_providers::Provider<ethers_providers::Http>,
    pub erc721_abi: abi::Abi,
    pub erc1155_abi: abi::Abi,
}

impl SafeEthers {
    pub async fn new(rpc_end_point: &str, chain_name: String) -> Result<SafeEthers> {
        let erc721_abi_str = fs::read_to_string("src/abis/erc721.json")?;
        let erc721_abi: abi::Abi = serde_json::from_str(&erc721_abi_str)?;

        let erc1155_abi_str = fs::read_to_string("src/abis/erc1155.json")?;
        let erc1155_abi: abi::Abi = serde_json::from_str(&erc1155_abi_str)?;

        let provider =
            ethers_providers::Provider::<ethers_providers::Http>::try_from(rpc_end_point)?;

        // this is for validating the rpcEndPoint, without this, it keep requesting to wrong rpc
        provider.get_chainid().await?;

        Ok(SafeEthers {
            chain_name,
            provider,
            erc721_abi,
            erc1155_abi,
        })
    }

    pub async fn safe_get_block_with_txs(
        &mut self,
        block_number: u64,
    ) -> Option<types::Block<response::Transaction>> {
        let get_block_with_txs_result = self.provider.get_block_with_txs(block_number).await;

        match get_block_with_txs_result {
            Err(e) => {
                tracing::error!(
                    "{} : Error when calling get_block_with_txs for block number {} : {}",
                    self.chain_name,
                    block_number,
                    e
                );

                None
            }
            Ok(option) => option,
        }
    }

    pub async fn safe_get_transaction_receipt(
        &mut self,
        tx_hash: types::H256,
    ) -> Option<types::TransactionReceipt> {
        let get_block_with_txs_result = self.provider.get_transaction_receipt(tx_hash).await;

        match get_block_with_txs_result {
            Err(e) => {
                tracing::error!(
                    "{} : Error when calling get_transaction_receipt of tx_hash {} : {}",
                    self.chain_name,
                    tx_hash,
                    e
                );

                None
            }
            Ok(option) => option,
        }
    }

    pub async fn is_nft_of(
        &mut self,
        nft_type: nft_types::NftTypes,
        contract_address: types::H160,
    ) -> anyhow::Result<bool> {
        let abi = match nft_type {
            nft_types::NftTypes::Erc721 => self.erc721_abi.clone(),
            nft_types::NftTypes::Erc1155 => self.erc1155_abi.clone(),
        };

        let contract = ethers_contract::Contract::new(contract_address, abi, self.provider.clone());

        let support_interface_result = contract
            .method::<_, bool>("supportsInterface", nft_type.interface_id_bytes())
            .expect("failed to contract.method in is_nft_of")
            .call()
            .await;

        match support_interface_result {
            Err(e) => self.proc_support_interface_error(e, nft_type, contract_address),
            Ok(o) => Ok(o),
        }
    }
    // if this returns Ok, do check for erc11155 too, if this retursns Err, don't do check for erc1155
    pub fn handle_no_support_interface_error(
        &mut self,
        nft_type: nft_types::NftTypes,
        contract_address: types::H160,
        error_string: String,
    ) -> anyhow::Result<bool> {
        tracing::info!("{} : No supportsInterface error, when calling supportsInterface with {} of contract {:#x} : {}",
                self.chain_name,
                nft_type.to_string(),
                contract_address,
                error_string);

        Err(anyhow!("no supportInterface"))
    }
    pub fn proc_support_interface_error(
        &mut self,
        whole_error: ethers_contract::ContractError<
            ethers_providers::Provider<ethers_providers::Http>,
        >,
        nft_type: nft_types::NftTypes,
        contract_address: types::H160,
    ) -> anyhow::Result<bool> {
        match whole_error {
            //when the contract doesn't have a supportInterface function
            ContractError::MiddlewareError(ProviderError::JsonRpcClientError(e))
                if e.to_string()
                    == "(code: -32000, message: invalid opcode: INVALID, data: None)"
                    || e.to_string()
                        == "(code: -32000, message: execution reverted, data: None)" =>
            {
                self.handle_no_support_interface_error(nft_type, contract_address, e.to_string())
            }
            // this error also means not having supportInterface. e.g :0x33dc5152d7590f99b222bd5891defdcd89c9370e
            ContractError::AbiError(AbiError::DecodingError(Error::InvalidName(e))) => {
                self.handle_no_support_interface_error(nft_type, contract_address, e)
            }
            // this error also means not having supportInterface. e.g :0xc3f2b402d2616d51109308ce80f6f209be270115
            ContractError::MiddlewareError(ProviderError::JsonRpcClientError(e))
                if &(e.to_string())[0..65]
                    == "(code: 3, message: execution reverted: Can't send to 0x00 address" =>
            {
                self.handle_no_support_interface_error(nft_type, contract_address, e.to_string())
            }
            //other error e.g:node is not responsing
            other_error => {
                tracing::error!("{} : Unexpected error occured, when calling supportsInterface with {} of contract {:#x} : {}",
                 self.chain_name,
                nft_type.to_string(),
                contract_address,
                other_error);

                Ok(false)
            }
        }
    }
}
