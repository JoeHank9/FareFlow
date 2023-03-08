use crate::Contract;
use crate::ContractExt;
use crate::Timestamp;

use near_sdk::serde::Serialize;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise, Balance};
use near_sdk::json_types::U128;

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Donation {
  pub account_id: AccountId, 
  pub total_amount: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CreditST {
  pub account_id: AccountId, 
  pub total_amount: U128,
}

#[near_bindgen]
impl Contract {
  #[payable] // Public - People can attach money
  pub fn deposit(&mut self, time: Timestamp) -> U128 {
    // Get who is calling the method and how much $NEAR they attached
    let donor: AccountId = env::predecessor_account_id();
    let deposit_amount: Balance = env::attached_deposit();

    let mut deposit_so_far = self.total_deposit.get(&donor).unwrap_or(0);

    let to_transfer: Balance = if deposit_so_far == 0 {
      // This is the user's first donation, lets register it, which increases storage
      assert!(deposit_amount > STORAGE_COST, "Attach at least {} yoctoNEAR", STORAGE_COST);

      // Subtract the storage cost to the amount to transfer
      deposit_amount - STORAGE_COST
    }else{
      deposit_amount
    };

    // Persist in storage the amount donated so far
    deposit_so_far += deposit_amount;
    self.total_deposit.insert(&donor, &deposit_so_far);
    self.internal_add_deposit_to_owner(&donor, &time);
    
    log!("Thank you {} for deposit {}! You donated a total of {}", donor.clone(), deposit_amount, deposit_so_far);
    
    // Send the money to the beneficiary
    Promise::new(self.beneficiary.clone()).transfer(to_transfer);

    // Return the total amount donated so far
    U128(deposit_so_far)
  }

  pub fn payment(&mut self, time: Timestamp, amount: String) -> U128 {
    // Get who is calling the method and how much $NEAR they attached
    let payer: AccountId = env::predecessor_account_id();
    let payment_amount: Balance = amount.parse().unwrap();

    let mut deposit_so_far = self.total_deposit.get(&payer).unwrap_or(0);

    assert!(payment_amount <= deposit_so_far, "You need to deposit first NEAR");

    // Persist in storage the amount donated so far
    deposit_so_far -= payment_amount;
    self.total_deposit.insert(&payer, &deposit_so_far);
    //self.internal_add_payment_to_owner(&payer, &time);
    
    log!("Gracias {} for pagar {}! Te queda un saldo total de {}", payer.clone(), payment_amount, deposit_so_far);

    // Return the total amount donated so far
    U128(deposit_so_far)
  }

}