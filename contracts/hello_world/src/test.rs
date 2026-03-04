#![cfg(test)]
use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test_hello() {
    let env = Env::default();
    let contract_id = env.register(HelloWorld, ());
    let client = HelloWorldClient::new(&env, &contract_id);

    let result = client.hello(&String::from_str(&env, "World"));
    assert_eq!(
        result,
        vec![&env, String::from_str(&env, "Hello"), String::from_str(&env, "World")]
    );
}
