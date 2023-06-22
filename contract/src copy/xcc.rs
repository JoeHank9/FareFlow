use crate::*;
use near_sdk::{ext_contract, Gas, Balance};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_transfer)]
pub trait ExtTransfer {
    fn ft_transfer(&self, receiver_id: AccountId, amount: String) -> String;
    fn nft_mint(&self, token_id: TokenId, receiver_id: AccountId, ) -> String;
}