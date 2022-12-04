use deposit::Deposit;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise};

mod deposit;
mod internal;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Depository {
    pub deposit_register: Vec<Deposit>,
    pub deposit_account: AccountId,
    pub even_attempt: bool,
}

// Implement the contract structure
#[near_bindgen]
impl Depository {
    #[init]
    pub fn new(deposit_account: AccountId) -> Self {
        Self::internal_new(deposit_account)
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate(deposit_account: AccountId) -> Self {
        Self {
            deposit_register: Vec::new(),
            deposit_account,
            even_attempt: true,
        }
    }

    #[payable]
    pub fn deposit(&mut self) -> bool {
        self.internal_deposit()
    }

    pub fn list(&self) -> Vec<Deposit> {
        self.internal_list()
    }
}

impl Default for Depository {
    fn default() -> Self {
        env::panic_str("Depository contract should be initialized before usage")
    }
}
/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    // use super::*;
}
