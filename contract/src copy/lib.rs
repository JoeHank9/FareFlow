use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::U128;
/// Raw type for timestamp in nanoseconds
pub type Timestamp = u64;
/// Raw type for 32 bytes of the hash.
pub type CryptoHash = [u8; 32];

use crate::internal::*;
pub use crate::xcc::*;
pub use crate::metadata::*;
pub use crate::deposit::*;
pub use crate::enumeration::*;

mod internal;
mod deposit;
mod xcc;
mod metadata;
mod enumeration;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub beneficiary: AccountId,
  pub total_deposit: UnorderedMap<AccountId, u128>,
  pub deposit_st: UnorderedMap<AccountId, u128>,
  pub deposit: UnorderedMap<Timestamp, Deposit>,
  pub deposit_per_owner: LookupMap<AccountId, UnorderedSet<Timestamp>>,
  pub metapoolcontract: String
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    DepositPerOwner,
    DepositPerOwnerInner { account_id_hash: CryptoHash },
}

impl Default for Contract {
  fn default() -> Self {
    Self{
      beneficiary: "v1.faucet.nonofficial.testnet".parse().unwrap(),
      metapoolcontract: "meta-v2.pool.testnet".parse().unwrap(),
      total_deposit: UnorderedMap::new(b"a"),
      deposit_st: UnorderedMap::new(b"b"),
      deposit: UnorderedMap::new(b"c"),
      deposit_per_owner: LookupMap::new(StorageKey::DepositPerOwner.try_to_vec().unwrap()),
    }
  }
}

#[near_bindgen]
impl Contract {
  #[init]
  #[private] // Public - but only callable by env::current_account_id()
  pub fn init(beneficiary: AccountId, metapoolcontract: String) -> Self {
    Self {
      beneficiary,
      metapoolcontract,
      deposit: UnorderedMap::new(b"d"),
      total_deposit: UnorderedMap::new(b"a"),
      deposit_st: UnorderedMap::new(b"b"),
      deposit_per_owner: LookupMap::new(StorageKey::DepositPerOwner.try_to_vec().unwrap()),
    }
  }

  // Public - beneficiary getter
  pub fn get_beneficiary(&self) -> AccountId {
    self.beneficiary.clone()
  }

  // Public - but only callable by env::current_account_id(). Sets the beneficiary
  #[private]
  pub fn change_beneficiary(&mut self, beneficiary: AccountId) {
    self.beneficiary = beneficiary;
  }
}