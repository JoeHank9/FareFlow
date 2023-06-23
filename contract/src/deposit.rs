use crate::Contract;
use crate::ContractExt;
use crate::Timestamp;
use crate::*;

use near_sdk::serde::Serialize;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise, Balance, Gas};
use near_sdk::json_types::U128;

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;
//const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(300000000000000);

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Donation {
  pub account_id: AccountId, 
  pub total_amount: U128,
}

#[near_bindgen]
impl Contract {
  #[payable] // Public - People can attach money
  pub fn deposit(&mut self) -> U128 {
    // Get who is calling the method and how much $NEAR they attached
    let donor: AccountId = env::predecessor_account_id();
    let time: Timestamp = env::block_timestamp();
    let deposit_amount: Balance = env::attached_deposit();
    let mut deposit_so_far = self.total_deposit.get(&donor).unwrap_or(0);
    let end_time_stake = time + 31556926000000000;

    log!("Tiempo recibido {}, tiempo al finalizar {}", time, end_time_stake);
    let to_transfer: Balance = if deposit_so_far == 0 {
      // This is the user's first donation, lets register it, which increases storage
      assert!(deposit_amount > 2000000000000000000000000, "Attach at least 2 NEARs");

      // Subtract the storage cost to the amount to transfer
      deposit_amount - STORAGE_COST
    }else{
      deposit_amount
    };

    // Send the money to the beneficiary
    Promise::new(self.beneficiary.clone()).transfer(to_transfer);

    // Persist in storage the amount donated so far
    deposit_so_far += deposit_amount;
    self.total_deposit.insert(&donor, &deposit_so_far);
    self.internal_add_deposit_to_owner(&donor, &time);
    self.timelocked.insert(&donor, &end_time_stake);

    //stake the deposit
    ext_transfer::ext(self.metapoolcontract.parse::<AccountId>().unwrap())
    .with_unused_gas_weight(300_000_000_000_000)
    .with_attached_deposit(deposit_amount)
    .deposit_and_stake();
    
    log!("Thank you {} for deposit {}! You donated a total of {}, your NEARs will be staked until {}", 
    donor.clone(), deposit_amount, deposit_so_far, end_time_stake.clone());

    // Return the total amount donated so far
    U128(deposit_so_far)
  }

  pub fn unstake(&mut self) -> U128 {

    let donor: AccountId = env::predecessor_account_id();
    let time: Timestamp = env::block_timestamp();
    let mut end_time_stake = self.timelocked.get(&donor).unwrap_or(0);
    let mut amount: Balance = self.total_deposit.get(&donor).unwrap_or(0);

    assert!(
      amount > 2000000000000000000000000,
      "Deposit at least 2 NEAR",
    );

    assert!(
      time >= end_time_stake,
      "locked until {}",end_time_stake
    );

    let mut float_amount: f64 = amount as f64;
    float_amount = float_amount * 1.03;
    amount = float_amount as u128;
    end_time_stake = end_time_stake + 259200000000000;

    // unstake the deposit
    ext_transfer::ext(self.metapoolcontract.parse::<AccountId>().unwrap())
    .with_unused_gas_weight(300_000_000_000_000)
    .unstake(amount);

    //log!("Tiempo recibido {}, tiempo al finalizar {}", &time, &end_time_stake);
    self.timelocked.insert(&donor, &end_time_stake);
    self.total_deposit.insert(&donor, &0);
    self.deposit_to_withdraw.insert(&donor, &amount);

    U128(amount)
  }

  pub fn withdraw(&mut self) -> U128 {
    
    let time: Timestamp = env::block_timestamp();
    let donor: AccountId = env::predecessor_account_id();
    let amount = self.deposit_to_withdraw.get(&donor).unwrap_or(0);
    let amount_unstaked = self.total_deposit.get(&donor).unwrap_or(0);
    let end_time_stake = self.timelocked.get(&donor).unwrap_or(0);

    assert!(
      amount == 0,
      "You don't have amount unstaked to withdraw",
    );

    assert!(
      amount_unstaked > 2000000000000000000000000,
      "Unstake at least 2 NEAR",
    );

    assert!(
      time >= end_time_stake,
      "locked until {}",end_time_stake
    );

    // withdraw the amount unstaked
    ext_transfer::ext(self.metapoolcontract.parse::<AccountId>().unwrap())
    .with_unused_gas_weight(300_000_000_000_000)
    .withdraw_unstaked();

    //transfer amount unstaked
    Promise::new(donor.clone()).transfer(amount_unstaked);

    U128(amount)

  }

}