#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_token() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    client.initialize(&admin);

    client.mint(&user1, &1000);
    assert_eq!(client.balance(&user1), 1000);

    client.transfer(&user1, &user2, &400);
    assert_eq!(client.balance(&user1), 600);
    assert_eq!(client.balance(&user2), 400);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_already_initialized() {
    let env = Env::default();
    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);
    client.initialize(&admin);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn test_transfer_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    client.initialize(&admin);
    client.mint(&user1, &100);
    client.transfer(&user1, &user2, &200);
}
