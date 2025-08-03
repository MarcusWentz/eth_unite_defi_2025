#![cfg(test)]

use crate::timelocks::{Timelocks, TimelocksClient};
use soroban_sdk::{testutils::Address as _, Address, Env, U256};

#[test]
fn test_timelocks_constructor() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Timelocks);
    let client = TimelocksClient::new(&env, &contract_id);

    // Test that the contract can be initialized
    let withdrawal_src_timelock = U256::from_u32(&env, 300);
    let public_withdrawal_src_timelock = U256::from_u32(&env, 600);
    let cancellation_src_timelock = U256::from_u32(&env, 900);
    let public_cancellation_src_timelock = U256::from_u32(&env, 1200);
    let withdrawal_dst_timelock = U256::from_u32(&env, 150);
    let public_withdrawal_dst_timelock = U256::from_u32(&env, 300);
    let cancellation_dst_timelock = U256::from_u32(&env, 450);
    let public_cancellation_dst_timelock = U256::from_u32(&env, 600);

    let timelocks = client.init(
        &withdrawal_src_timelock,
        &public_withdrawal_src_timelock,
        &cancellation_src_timelock,
        &public_cancellation_src_timelock,
        &withdrawal_dst_timelock,
        &public_withdrawal_dst_timelock,
        &cancellation_dst_timelock,
        &public_cancellation_dst_timelock,
    );

    // Verify timelocks are properly set
    assert_eq!(timelocks.withdrawal_src_timelock, withdrawal_src_timelock);
    assert_eq!(timelocks.public_withdrawal_src_timelock, public_withdrawal_src_timelock);
    assert_eq!(timelocks.cancellation_src_timelock, cancellation_src_timelock);
    assert_eq!(timelocks.public_cancellation_src_timelock, public_cancellation_src_timelock);
    assert_eq!(timelocks.withdrawal_dst_timelock, withdrawal_dst_timelock);
    assert_eq!(timelocks.public_withdrawal_dst_timelock, public_withdrawal_dst_timelock);
    assert_eq!(timelocks.cancellation_dst_timelock, cancellation_dst_timelock);
    assert_eq!(timelocks.public_cancellation_dst_timelock, public_cancellation_dst_timelock);
}

#[test]
fn test_set_deployed_at() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Timelocks);
    let client = TimelocksClient::new(&env, &contract_id);

    // Initialize timelocks
    let withdrawal_src_timelock = U256::from_u32(&env, 300);
    let public_withdrawal_src_timelock = U256::from_u32(&env, 600);
    let cancellation_src_timelock = U256::from_u32(&env, 900);
    let public_cancellation_src_timelock = U256::from_u32(&env, 1200);
    let withdrawal_dst_timelock = U256::from_u32(&env, 150);
    let public_withdrawal_dst_timelock = U256::from_u32(&env, 300);
    let cancellation_dst_timelock = U256::from_u32(&env, 450);
    let public_cancellation_dst_timelock = U256::from_u32(&env, 600);

    let _timelocks = client.init(
        &withdrawal_src_timelock,
        &public_withdrawal_src_timelock,
        &cancellation_src_timelock,
        &public_cancellation_src_timelock,
        &withdrawal_dst_timelock,
        &public_withdrawal_dst_timelock,
        &cancellation_dst_timelock,
        &public_cancellation_dst_timelock,
    );

    // Test setting deployed_at timestamp
    let deployed_at = U256::from_u32(&env, 1000);
    client.set_deployed_at(&deployed_at);

    // Verify deployed_at was set
    let retrieved_deployed_at = client.get_deployed_at();
    assert_eq!(retrieved_deployed_at, deployed_at);
}

#[test]
fn test_rescue_start() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Timelocks);
    let client = TimelocksClient::new(&env, &contract_id);

    // Initialize timelocks
    let withdrawal_src_timelock = U256::from_u32(&env, 300);
    let public_withdrawal_src_timelock = U256::from_u32(&env, 600);
    let cancellation_src_timelock = U256::from_u32(&env, 900);
    let public_cancellation_src_timelock = U256::from_u32(&env, 1200);
    let withdrawal_dst_timelock = U256::from_u32(&env, 150);
    let public_withdrawal_dst_timelock = U256::from_u32(&env, 300);
    let cancellation_dst_timelock = U256::from_u32(&env, 450);
    let public_cancellation_dst_timelock = U256::from_u32(&env, 600);

    let _timelocks = client.init(
        &withdrawal_src_timelock,
        &public_withdrawal_src_timelock,
        &cancellation_src_timelock,
        &public_cancellation_src_timelock,
        &withdrawal_dst_timelock,
        &public_withdrawal_dst_timelock,
        &cancellation_dst_timelock,
        &public_cancellation_dst_timelock,
    );

    // Set deployed_at timestamp
    let deployed_at = U256::from_u32(&env, 1000);
    client.set_deployed_at(&deployed_at);

    // Test rescue_start calculation
    let rescue_start = client.rescue_start();
    
    // Verify rescue_start is calculated correctly (deployed_at + cancellation_src_timelock)
    let expected_rescue_start = deployed_at + cancellation_src_timelock;
    assert_eq!(rescue_start, expected_rescue_start);
}

#[test]
fn test_get_timelocks() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Timelocks);
    let client = TimelocksClient::new(&env, &contract_id);

    // Initialize timelocks with specific values
    let withdrawal_src_timelock = U256::from_u32(&env, 300);
    let public_withdrawal_src_timelock = U256::from_u32(&env, 600);
    let cancellation_src_timelock = U256::from_u32(&env, 900);
    let public_cancellation_src_timelock = U256::from_u32(&env, 1200);
    let withdrawal_dst_timelock = U256::from_u32(&env, 150);
    let public_withdrawal_dst_timelock = U256::from_u32(&env, 300);
    let cancellation_dst_timelock = U256::from_u32(&env, 450);
    let public_cancellation_dst_timelock = U256::from_u32(&env, 600);

    let timelocks = client.init(
        &withdrawal_src_timelock,
        &public_withdrawal_src_timelock,
        &cancellation_src_timelock,
        &public_cancellation_src_timelock,
        &withdrawal_dst_timelock,
        &public_withdrawal_dst_timelock,
        &cancellation_dst_timelock,
        &public_cancellation_dst_timelock,
    );

    // Test getting timelocks
    let retrieved_timelocks = client.get();

    // Verify all timelock values are correctly retrieved
    assert_eq!(retrieved_timelocks.withdrawal_src_timelock, withdrawal_src_timelock);
    assert_eq!(retrieved_timelocks.public_withdrawal_src_timelock, public_withdrawal_src_timelock);
    assert_eq!(retrieved_timelocks.cancellation_src_timelock, cancellation_src_timelock);
    assert_eq!(retrieved_timelocks.public_cancellation_src_timelock, public_cancellation_src_timelock);
    assert_eq!(retrieved_timelocks.withdrawal_dst_timelock, withdrawal_dst_timelock);
    assert_eq!(retrieved_timelocks.public_withdrawal_dst_timelock, public_withdrawal_dst_timelock);
    assert_eq!(retrieved_timelocks.cancellation_dst_timelock, cancellation_dst_timelock);
    assert_eq!(retrieved_timelocks.public_cancellation_dst_timelock, public_cancellation_dst_timelock);
}

#[test]
fn test_timelock_relationships() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Timelocks);
    let client = TimelocksClient::new(&env, &contract_id);

    // Initialize timelocks with realistic values
    let withdrawal_src_timelock = U256::from_u32(&env, 300);
    let public_withdrawal_src_timelock = U256::from_u32(&env, 600);
    let cancellation_src_timelock = U256::from_u32(&env, 900);
    let public_cancellation_src_timelock = U256::from_u32(&env, 1200);
    let withdrawal_dst_timelock = U256::from_u32(&env, 150);
    let public_withdrawal_dst_timelock = U256::from_u32(&env, 300);
    let cancellation_dst_timelock = U256::from_u32(&env, 450);
    let public_cancellation_dst_timelock = U256::from_u32(&env, 600);

    let _timelocks = client.init(
        &withdrawal_src_timelock,
        &public_withdrawal_src_timelock,
        &cancellation_src_timelock,
        &public_cancellation_src_timelock,
        &withdrawal_dst_timelock,
        &public_withdrawal_dst_timelock,
        &cancellation_dst_timelock,
        &public_cancellation_dst_timelock,
    );

    // Test that timelock relationships make sense
    // Public timelocks should be longer than regular timelocks
    assert!(public_withdrawal_src_timelock > withdrawal_src_timelock);
    assert!(public_cancellation_src_timelock > cancellation_src_timelock);
    assert!(public_withdrawal_dst_timelock > withdrawal_dst_timelock);
    assert!(public_cancellation_dst_timelock > cancellation_dst_timelock);

    // Cancellation timelocks should be longer than withdrawal timelocks
    assert!(cancellation_src_timelock > withdrawal_src_timelock);
    assert!(cancellation_dst_timelock > withdrawal_dst_timelock);
}

#[test]
fn test_edge_cases() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Timelocks);
    let client = TimelocksClient::new(&env, &contract_id);

    // Test with maximum values
    let max_timelock = U256::from_u128(&env, u128::MAX);
    
    let timelocks = client.init(
        &max_timelock,
        &max_timelock,
        &max_timelock,
        &max_timelock,
        &max_timelock,
        &max_timelock,
        &max_timelock,
        &max_timelock,
    );

    // Verify maximum values are handled
    assert_eq!(timelocks.withdrawal_src_timelock, max_timelock);
    assert_eq!(timelocks.public_withdrawal_src_timelock, max_timelock);
    assert_eq!(timelocks.cancellation_src_timelock, max_timelock);
    assert_eq!(timelocks.public_cancellation_src_timelock, max_timelock);
    assert_eq!(timelocks.withdrawal_dst_timelock, max_timelock);
    assert_eq!(timelocks.public_withdrawal_dst_timelock, max_timelock);
    assert_eq!(timelocks.cancellation_dst_timelock, max_timelock);
    assert_eq!(timelocks.public_cancellation_dst_timelock, max_timelock);
}

#[test]
fn test_zero_values() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Timelocks);
    let client = TimelocksClient::new(&env, &contract_id);

    // Test with zero values
    let zero_timelock = U256::from_u32(&env, 0);
    
    let timelocks = client.init(
        &zero_timelock,
        &zero_timelock,
        &zero_timelock,
        &zero_timelock,
        &zero_timelock,
        &zero_timelock,
        &zero_timelock,
        &zero_timelock,
    );

    // Verify zero values are handled
    assert_eq!(timelocks.withdrawal_src_timelock, zero_timelock);
    assert_eq!(timelocks.public_withdrawal_src_timelock, zero_timelock);
    assert_eq!(timelocks.cancellation_src_timelock, zero_timelock);
    assert_eq!(timelocks.public_cancellation_src_timelock, zero_timelock);
    assert_eq!(timelocks.withdrawal_dst_timelock, zero_timelock);
    assert_eq!(timelocks.public_withdrawal_dst_timelock, zero_timelock);
    assert_eq!(timelocks.cancellation_dst_timelock, zero_timelock);
    assert_eq!(timelocks.public_cancellation_dst_timelock, zero_timelock);
}

#[test]
fn test_multiple_deployed_at_updates() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Timelocks);
    let client = TimelocksClient::new(&env, &contract_id);

    // Initialize timelocks
    let withdrawal_src_timelock = U256::from_u32(&env, 300);
    let public_withdrawal_src_timelock = U256::from_u32(&env, 600);
    let cancellation_src_timelock = U256::from_u32(&env, 900);
    let public_cancellation_src_timelock = U256::from_u32(&env, 1200);
    let withdrawal_dst_timelock = U256::from_u32(&env, 150);
    let public_withdrawal_dst_timelock = U256::from_u32(&env, 300);
    let cancellation_dst_timelock = U256::from_u32(&env, 450);
    let public_cancellation_dst_timelock = U256::from_u32(&env, 600);

    let _timelocks = client.init(
        &withdrawal_src_timelock,
        &public_withdrawal_src_timelock,
        &cancellation_src_timelock,
        &public_cancellation_src_timelock,
        &withdrawal_dst_timelock,
        &public_withdrawal_dst_timelock,
        &cancellation_dst_timelock,
        &public_cancellation_dst_timelock,
    );

    // Test multiple deployed_at updates
    let deployed_at1 = U256::from_u32(&env, 1000);
    client.set_deployed_at(&deployed_at1);
    assert_eq!(client.get_deployed_at(), deployed_at1);

    let deployed_at2 = U256::from_u32(&env, 2000);
    client.set_deployed_at(&deployed_at2);
    assert_eq!(client.get_deployed_at(), deployed_at2);

    // Verify rescue_start is updated accordingly
    let rescue_start = client.rescue_start();
    let expected_rescue_start = deployed_at2 + cancellation_src_timelock;
    assert_eq!(rescue_start, expected_rescue_start);
}

#[test]
fn test_timelock_calculation_accuracy() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Timelocks);
    let client = TimelocksClient::new(&env, &contract_id);

    // Initialize timelocks with specific values
    let withdrawal_src_timelock = U256::from_u32(&env, 300);
    let public_withdrawal_src_timelock = U256::from_u32(&env, 600);
    let cancellation_src_timelock = U256::from_u32(&env, 900);
    let public_cancellation_src_timelock = U256::from_u32(&env, 1200);
    let withdrawal_dst_timelock = U256::from_u32(&env, 150);
    let public_withdrawal_dst_timelock = U256::from_u32(&env, 300);
    let cancellation_dst_timelock = U256::from_u32(&env, 450);
    let public_cancellation_dst_timelock = U256::from_u32(&env, 600);

    let _timelocks = client.init(
        &withdrawal_src_timelock,
        &public_withdrawal_src_timelock,
        &cancellation_src_timelock,
        &public_cancellation_src_timelock,
        &withdrawal_dst_timelock,
        &public_withdrawal_dst_timelock,
        &cancellation_dst_timelock,
        &public_cancellation_dst_timelock,
    );

    // Test rescue_start calculation with different deployed_at values
    let test_deployed_at_values = [
        U256::from_u32(&env, 1000),
        U256::from_u32(&env, 5000),
        U256::from_u32(&env, 10000),
    ];

    for deployed_at in test_deployed_at_values {
        client.set_deployed_at(&deployed_at);
        let rescue_start = client.rescue_start();
        let expected_rescue_start = deployed_at + cancellation_src_timelock;
        assert_eq!(rescue_start, expected_rescue_start);
    }
}

#[test]
fn test_timelock_consistency() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Timelocks);
    let client = TimelocksClient::new(&env, &contract_id);

    // Initialize timelocks
    let withdrawal_src_timelock = U256::from_u32(&env, 300);
    let public_withdrawal_src_timelock = U256::from_u32(&env, 600);
    let cancellation_src_timelock = U256::from_u32(&env, 900);
    let public_cancellation_src_timelock = U256::from_u32(&env, 1200);
    let withdrawal_dst_timelock = U256::from_u32(&env, 150);
    let public_withdrawal_dst_timelock = U256::from_u32(&env, 300);
    let cancellation_dst_timelock = U256::from_u32(&env, 450);
    let public_cancellation_dst_timelock = U256::from_u32(&env, 600);

    let timelocks1 = client.init(
        &withdrawal_src_timelock,
        &public_withdrawal_src_timelock,
        &cancellation_src_timelock,
        &public_cancellation_src_timelock,
        &withdrawal_dst_timelock,
        &public_withdrawal_dst_timelock,
        &cancellation_dst_timelock,
        &public_cancellation_dst_timelock,
    );

    // Test that multiple calls to get() return consistent values
    let timelocks2 = client.get();
    let timelocks3 = client.get();

    assert_eq!(timelocks1.withdrawal_src_timelock, timelocks2.withdrawal_src_timelock);
    assert_eq!(timelocks2.withdrawal_src_timelock, timelocks3.withdrawal_src_timelock);
    assert_eq!(timelocks1.public_withdrawal_src_timelock, timelocks2.public_withdrawal_src_timelock);
    assert_eq!(timelocks2.public_withdrawal_src_timelock, timelocks3.public_withdrawal_src_timelock);
} 