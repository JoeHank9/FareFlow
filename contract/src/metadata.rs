use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Deposit {
    pub deposit: u64,
    pub mxn: u64,
}