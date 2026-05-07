#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

// Define the keys used to store data in the Storage
#[contracttype]
pub enum DataKey {
    TotalFund,              // Key to store the total amount of the group fund
    UserBalance(Address),   // Key to store the contribution balance of a specific address
}

#[contract]
pub struct GroupFundContract;

#[contractimpl]
impl GroupFundContract {
    /// Deposit function
    /// Stores the amount of tokens contributed by a user (Address) to the group fund.
    pub fn deposit(env: Env, from: Address, amount: i128) {
        // Require authentication from the caller to ensure no one can 
        // arbitrarily forge contributions under someone else's name.
        from.require_auth();

        // Validation: The deposit amount must be greater than 0
        if amount <= 0 {
            panic!("Deposit amount must be greater than 0");
        }

        // 1. Update the individual balance of the sender
        let user_key = DataKey::UserBalance(from.clone());
        // Get the current balance from persistent storage, default to 0 if it doesn't exist
        let mut user_balance: i128 = env.storage().persistent().get(&user_key).unwrap_or(0);
        user_balance += amount;
        // Save the updated balance
        env.storage().persistent().set(&user_key, &user_balance);

        // 2. Update the total amount of the entire group fund
        let mut total_fund: i128 = env.storage().persistent().get(&DataKey::TotalFund).unwrap_or(0);
        total_fund += amount;
        env.storage().persistent().set(&DataKey::TotalFund, &total_fund);
    }

    /// Get the contribution balance of a specific user
    pub fn get_user_balance(env: Env, user: Address) -> i128 {
        let user_key = DataKey::UserBalance(user);
        env.storage().persistent().get(&user_key).unwrap_or(0)
    }

    /// Get the total fund currently recorded by the contract
    pub fn get_total_fund(env: Env) -> i128 {
        env.storage().persistent().get(&DataKey::TotalFund).unwrap_or(0)
    }
}