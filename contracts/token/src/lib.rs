#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
    Admin,
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    /// Initializes the token with an administrator
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Returns the balance of an account
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().instance().get(&DataKey::Balance(id)).unwrap_or(0)
    }

    /// Mints new tokens (admin only)
    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("not initialized");
        admin.require_auth();
        
        if amount < 0 {
            panic!("negative amount is not allowed");
        }

        let mut balance = Self::balance(env.clone(), to.clone());
        balance += amount;
        env.storage().instance().set(&DataKey::Balance(to), &balance);
    }

    /// Transfers tokens from one account to another
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        if amount < 0 {
            panic!("negative amount is not allowed");
        }

        let mut from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            panic!("insufficient balance");
        }

        from_balance -= amount;
        env.storage().instance().set(&DataKey::Balance(from), &from_balance);

        let mut to_balance = Self::balance(env.clone(), to.clone());
        to_balance += amount;
        env.storage().instance().set(&DataKey::Balance(to), &to_balance);
    }
}

#[cfg(test)]
mod test;
