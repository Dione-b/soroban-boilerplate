#![cfg(test)]
use soroban_sdk::{testutils::Address as _, Address, Env};
use hello_world::{HelloWorld, HelloWorldClient};
use increment::{Increment, IncrementClient};
use token::{TokenContract, TokenContractClient};

#[test]
fn test_cross_contract_interaction() {
    let env = Env::default();
    env.mock_all_auths();

    // Deploy contracts directly via struct registration for testing
    let hello_id = env.register(HelloWorld, ());
    let inc_id = env.register(Increment, ());
    let token_id = env.register(TokenContract, ());

    let hello_client = HelloWorldClient::new(&env, &hello_id);
    let inc_client = IncrementClient::new(&env, &inc_id);
    let token_client = TokenContractClient::new(&env, &token_id);

    // Example: call hello
    let user = Address::generate(&env);
    let greeting = hello_client.hello(&soroban_sdk::String::from_str(&env, "Integration"));
    assert!(!greeting.is_empty());

    // Example: increment counter
    assert_eq!(inc_client.increment(), 1);
    assert_eq!(inc_client.increment(), 2);

    // Example: mint and transfer tokens
    token_client.initialize(&user);
    token_client.mint(&user, &1000);
    assert_eq!(token_client.balance(&user), 1000);
}
