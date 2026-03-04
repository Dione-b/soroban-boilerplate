#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, Env};

#[contracttype]
pub enum DataKey {
    Counter,
}

#[contract]
pub struct Increment;

#[contractimpl]
impl Increment {
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0);
        log!(&env, "count: {}", count);
        count += 1;
        env.storage().instance().set(&DataKey::Counter, &count);
        env.storage().instance().extend_ttl(50, 100);
        count
    }

    pub fn get_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Counter).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
