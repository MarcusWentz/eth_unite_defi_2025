#![cfg(test)]

use crate::base_escrow::BaseEscrow;
use crate::Immutables;
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, U256};

#[test]
#[should_panic(expected = "Not a taker")]
fn test_only_taker_unauthorized_access() {
    let env = Env::default();
    
    // Create test addresses
    let authorized_taker = Address::generate(&env);
    let unauthorized_user = Address::generate(&env);
    let maker = Address::generate(&env);
    
    // Create test immutables with authorized taker
    let immutables = Immutables {
        order_hash: BytesN::from_array(&env, &[1u8; 32]),
        hashlock: BytesN::from_array(&env, &[2u8; 32]),
        maker: maker.clone(),
        taker: authorized_taker.clone(),
        token: Address::generate(&env),
        amount: 1000000000000000000u128,
        safety_deposit: 100000000000000000u128,
        timelocks: U256::from_u32(&env, 1000),
    };
    
    // Set unauthorized user as sender
    env.storage()
        .persistent()
        .set(&soroban_sdk::symbol_short!("sender"), &unauthorized_user);
    
    // This should panic with "Not a taker"
    let _ = <dyn BaseEscrow>::only_taker(env, immutables);
}

#[test]
fn test_only_taker_authorized_access() {
    let env = Env::default();
    
    // Create test addresses
    let authorized_taker = Address::generate(&env);
    let maker = Address::generate(&env);
    
    // Create test immutables
    let immutables = Immutables {
        order_hash: BytesN::from_array(&env, &[1u8; 32]),
        hashlock: BytesN::from_array(&env, &[2u8; 32]),
        maker: maker.clone(),
        taker: authorized_taker.clone(),
        token: Address::generate(&env),
        amount: 1000000000000000000u128,
        safety_deposit: 100000000000000000u128,
        timelocks: U256::from_u32(&env, 1000),
    };
    
    // Set authorized taker as sender
    env.storage()
        .persistent()
        .set(&soroban_sdk::symbol_short!("sender"), &authorized_taker);
    
    // This should succeed
    let result = <dyn BaseEscrow>::only_taker(env, immutables);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Invalid secret")]
fn test_only_valid_secret_invalid_secret() {
    let env = Env::default();
    
    let maker = Address::generate(&env);
    let taker = Address::generate(&env);
    
    // Create test immutables with a specific hashlock
    let immutables = Immutables {
        order_hash: BytesN::from_array(&env, &[1u8; 32]),
        hashlock: BytesN::from_array(&env, &[2u8; 32]), // Specific hashlock
        maker: maker.clone(),
        taker: taker.clone(),
        token: Address::generate(&env),
        amount: 1000000000000000000u128,
        safety_deposit: 100000000000000000u128,
        timelocks: U256::from_u32(&env, 1000),
    };
    
    // Use wrong secret that doesn't match the hashlock
    let wrong_secret = BytesN::from_array(&env, &[255u8; 32]);
    
    // This should panic with "Invalid secret"
    let _ = <dyn BaseEscrow>::only_valid_secret(env, wrong_secret, immutables);
}

#[test]
fn test_only_valid_secret_valid_secret() {
    let env = Env::default();
    
    let maker = Address::generate(&env);
    let taker = Address::generate(&env);
    
    // Create a secret and compute its hashlock
    let secret = BytesN::from_array(&env, &[42u8; 32]);
    let secret_xdr = secret.as_object().to_xdr(&env);
    let hashlock = env.crypto().keccak256(&secret_xdr).to_bytes();
    
    // Create test immutables with the correct hashlock
    let immutables = Immutables {
        order_hash: BytesN::from_array(&env, &[1u8; 32]),
        hashlock: hashlock,
        maker: maker.clone(),
        taker: taker.clone(),
        token: Address::generate(&env),
        amount: 1000000000000000000u128,
        safety_deposit: 100000000000000000u128,
        timelocks: U256::from_u32(&env, 1000),
    };
    
    // This should succeed
    let result = <dyn BaseEscrow>::only_valid_secret(env, secret, immutables);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Invalid time")]
fn test_only_after_too_early() {
    let env = Env::default();
    
    // Set current time to 1000
    env.ledger().set_timestamp(1000);
    
    // Try to access before time 2000
    let start_time = U256::from_u32(&env, 2000);
    
    // This should panic with "Invalid time"
    let _ = <dyn BaseEscrow>::only_after(env, start_time);
}

#[test]
fn test_only_after_valid_time() {
    let env = Env::default();
    
    // Set current time to 3000
    env.ledger().set_timestamp(3000);
    
    // Try to access after time 2000
    let start_time = U256::from_u32(&env, 2000);
    
    // This should succeed
    let result = <dyn BaseEscrow>::only_after(env, start_time);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Invalid time")]
fn test_only_before_too_late() {
    let env = Env::default();
    
    // Set current time to 3000
    env.ledger().set_timestamp(3000);
    
    // Try to access after deadline 2000
    let stop_time = U256::from_u32(&env, 2000);
    
    // This should panic with "Invalid time"
    let _ = <dyn BaseEscrow>::only_before(env, stop_time);
}

#[test]
fn test_only_before_valid_time() {
    let env = Env::default();
    
    // Set current time to 1000
    env.ledger().set_timestamp(1000);
    
    // Try to access before deadline 2000
    let stop_time = U256::from_u32(&env, 2000);
    
    // This should succeed
    let result = <dyn BaseEscrow>::only_before(env, stop_time);
    assert!(result.is_ok());
} 