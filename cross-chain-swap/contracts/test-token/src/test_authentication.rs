#![cfg(test)]

use crate::test_token::{TestToken, TestTokenClient};
use soroban_sdk::{testutils::Address as _, Address, Env, U256};

#[test]
#[should_panic(expected = "AccessDenied")]
fn test_transfer_unauthorized_from() {
    let env = Env::default();
    
    // Create test addresses
    let authorized_from = Address::generate(&env);
    let unauthorized_from = Address::generate(&env);
    let to = Address::generate(&env);
    
    // Deploy token contract
    let contract_id = env.register(TestToken, ());
    let client = TestTokenClient::new(&env, &contract_id);
    
    // Set unauthorized user as sender
    env.storage()
        .persistent()
        .set(&soroban_sdk::symbol_short!("sender"), &unauthorized_from);
    
    // This should panic with "AccessDenied" because unauthorized_from is not authorized
    let _ = client.transfer(&unauthorized_from, &to, &1000);
}

#[test]
fn test_transfer_authorized_from() {
    let env = Env::default();
    
    // Create test addresses
    let authorized_from = Address::generate(&env);
    let to = Address::generate(&env);
    
    // Deploy token contract
    let contract_id = env.register(TestToken, ());
    let client = TestTokenClient::new(&env, &contract_id);
    
    // Set authorized user as sender
    env.storage()
        .persistent()
        .set(&soroban_sdk::symbol_short!("sender"), &authorized_from);
    
    // Mock all auths for testing
    env.mock_all_auths();
    
    // This should succeed because authorized_from is authorized
    let result = client.transfer(&authorized_from, &to, &1000);
    assert_eq!(result, true);
}

#[test]
#[should_panic(expected = "AccessDenied")]
fn test_approve_unauthorized_owner() {
    let env = Env::default();
    
    // Create test addresses
    let authorized_owner = Address::generate(&env);
    let unauthorized_owner = Address::generate(&env);
    let spender = Address::generate(&env);
    
    // Deploy token contract
    let contract_id = env.register(TestToken, ());
    let client = TestTokenClient::new(&env, &contract_id);
    
    // Set unauthorized user as sender
    env.storage()
        .persistent()
        .set(&soroban_sdk::symbol_short!("sender"), &unauthorized_owner);
    
    // This should panic with "AccessDenied" because unauthorized_owner is not authorized
    let _ = client.approve(&unauthorized_owner, &spender, &1000);
}

#[test]
fn test_approve_authorized_owner() {
    let env = Env::default();
    
    // Create test addresses
    let authorized_owner = Address::generate(&env);
    let spender = Address::generate(&env);
    
    // Deploy token contract
    let contract_id = env.register(TestToken, ());
    let client = TestTokenClient::new(&env, &contract_id);
    
    // Set authorized user as sender
    env.storage()
        .persistent()
        .set(&soroban_sdk::symbol_short!("sender"), &authorized_owner);
    
    // Mock all auths for testing
    env.mock_all_auths();
    
    // This should succeed because authorized_owner is authorized
    let result = client.approve(&authorized_owner, &spender, &1000);
    assert_eq!(result, true);
}

#[test]
#[should_panic(expected = "AccessDenied")]
fn test_set_admin_unauthorized_admin() {
    let env = Env::default();
    
    // Create test addresses
    let authorized_admin = Address::generate(&env);
    let unauthorized_admin = Address::generate(&env);
    let new_admin = Address::generate(&env);
    
    // Deploy token contract
    let contract_id = env.register(TestToken, ());
    let client = TestTokenClient::new(&env, &contract_id);
    
    // Set unauthorized user as sender
    env.storage()
        .persistent()
        .set(&soroban_sdk::symbol_short!("sender"), &unauthorized_admin);
    
    // This should panic with "AccessDenied" because unauthorized_admin is not authorized
    let _ = client.set_admin(&unauthorized_admin, &new_admin);
}

#[test]
fn test_set_admin_authorized_admin() {
    let env = Env::default();
    
    // Create test addresses
    let authorized_admin = Address::generate(&env);
    let new_admin = Address::generate(&env);
    
    // Deploy token contract
    let contract_id = env.register(TestToken, ());
    let client = TestTokenClient::new(&env, &contract_id);
    
    // Set authorized user as sender
    env.storage()
        .persistent()
        .set(&soroban_sdk::symbol_short!("sender"), &authorized_admin);
    
    // Mock all auths for testing
    env.mock_all_auths();
    
    // This should succeed because authorized_admin is authorized
    let _ = client.set_admin(&authorized_admin, &new_admin);
    
    // Verify the admin was set
    let current_admin = client.admin();
    assert_eq!(current_admin, new_admin);
} 