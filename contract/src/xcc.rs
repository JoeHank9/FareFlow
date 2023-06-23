use crate::*;
use near_sdk::{ext_contract};

// const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
// const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
// const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);

#[ext_contract(ext_transfer)]
pub trait ExtTransfer {
    //fn ft_transfer(&self, receiver_id: AccountId, amount: String) -> String;
    fn deposit_and_stake(&self) -> String;
    fn unstake(&self, amount: u128) -> String;
    fn withdraw_unstaked(&self) -> String;
}