use crate::*;

#[near_bindgen]
impl Contract {

    // Public - get donation by account ID
  pub fn get_donation_for_account(&self, account_id: AccountId) -> Donation {
    Donation {
      account_id: account_id.clone(),
      total_amount: U128(self.total_deposit.get(&account_id).unwrap_or(0))
    }
  }

    // Public - get donation by account ID
    pub fn get_credit_for_account(&self, account_id: AccountId) -> CreditST {
      CreditST {
        account_id: account_id.clone(),
        total_amount: U128(self.total_deposit.get(&account_id).unwrap_or(0))
      }
    }

  // Public - get total number of donors
  pub fn number_of_donors(&self) -> u64 {
    self.total_deposit.len()
  }

  // Public - paginate through all donations on the contract
  pub fn get_donations(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Donation> {
    //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
    let start = u128::from(from_index.unwrap_or(U128(0)));

    //iterate through donation
    self.total_deposit.keys()
      //skip to the index we specified in the start variable
      .skip(start as usize) 
      //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
      .take(limit.unwrap_or(50) as usize) 
      .map(|account| self.get_donation_for_account(account))
      //since we turned map into an iterator, we need to turn it back into a vector to return
      .collect()
  }
    //Query for the total supply of NFTs on the contract
    // pub fn nft_total_supply(&self) -> U128 {
    //     //return the length of the token metadata by ID
    //     U128(self.token_metadata_by_id.len() as u128)
    // }

    //Query for nft tokens on the contract regardless of the owner using pagination
    // pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
    //     //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
    //     let start = u128::from(from_index.unwrap_or(U128(0)));

    //     //iterate through each token using an iterator
    //     self.token_metadata_by_id.keys_as_vector().iter()
    //         //skip to the index we specified in the start variable
    //         .skip(start as usize) 
    //         //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
    //         .take(limit.unwrap_or(50) as usize) 
    //         //we'll map the token IDs which are strings into Json Tokens
    //         .map(|token_id| self.nft_token(token_id.clone()).unwrap())
    //         //since we turned the keys into an iterator, we need to turn it back into a vector to return
    //         .collect()
    // }

    // //get the total supply of NFTs for a given owner
    pub fn deposit_supply_for_owner(
        &self,
        account_id: AccountId,
    ) -> U128 {
        //get the set of tokens for the passed in owner
        let deposit_for_owner_set = self.deposit_per_owner.get(&account_id);

        //if there is some set of tokens, we'll return the length as a U128
        if let Some(deposit_for_owner_set) = deposit_for_owner_set {
            U128(deposit_for_owner_set.len() as u128)
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            U128(0)
        }
    }

    // //Query for all the tokens for an owner
    // pub fn deposit_for_owner(
    //     &self,
    //     account_id: AccountId,
    //     from_index: Option<U128>,
    //     limit: Option<u64>,
    // ) -> Vec <Donation> {
    //     //get the set of tokens for the passed in owner
    //     let deposit_for_owner_set = self.deposit_per_owner.get(&account_id);
    //     //if there is some set of tokens, we'll set the tokens variable equal to that set
    //     let tokens = if let Some(deposit_for_owner_set) = deposit_for_owner_set {
    //         deposit_for_owner_set
    //     } else {
    //         //if there is no set of tokens, we'll simply return an empty vector. 
    //         return vec![];
    //     };

    //     //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
    //     let start = u128::from(from_index.unwrap_or(U128(0)));

    //     //iterate through the keys vector
    //     tokens.iter()
    //         //skip to the index we specified in the start variable
    //         .skip(start as usize) 
    //         //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
    //         .take(limit.unwrap_or(50) as usize) 
    //         //we'll map the token IDs which are strings into Json Tokens
    //         .map(|token_id| self.nft_token(token_id.clone()).unwrap())
    //         //since we turned the keys into an iterator, we need to turn it back into a vector to return
    //         .collect()
    // }
    
}


