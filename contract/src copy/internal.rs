use crate::*;
use near_sdk::{CryptoHash};
use std::mem::size_of;

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

impl Contract {
    //add a token to the set of tokens an owner has
    pub(crate) fn internal_add_deposit_to_owner(
        &mut self,
        account_id: &AccountId,
        timestamp: &Timestamp,
    ) {
        //get the set of tokens for the given account
        let mut deposit_set = self.deposit_per_owner.get(account_id).unwrap_or_else(|| {
            //if the account doesn't have any tokens, we create a new unordered set
            UnorderedSet::new(
                StorageKey::DepositPerOwnerInner {
                    //we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //we insert the token ID into the set
        deposit_set.insert(timestamp);

        //we insert that set for the given account ID. 
        self.deposit_per_owner.insert(account_id, &deposit_set);
    }

    
    
} 