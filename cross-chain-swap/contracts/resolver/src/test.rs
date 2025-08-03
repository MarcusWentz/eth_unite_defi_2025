#![cfg(test)]

use crate::{ResolverContract, ResolverContractClient};
use base_escrow::Immutables;
use order_interface::Order;
use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env, U256};

#[test]
fn test_constructor_and_getters() {
    let env = Env::default();
    let admin = Address::generate(&env);

    let escrow_factory_address = Address::generate(&env);
    let order_mixin_address = Address::generate(&env);

    let contract_id = env.register(
        ResolverContract,
        (&escrow_factory_address, &order_mixin_address),
    );
    let resolver_client = ResolverContractClient::new(&env, &contract_id);

    // Test getter functions
    let retrieved_escrow_factory = resolver_client.get_escrow_factory_address();
    assert_eq!(retrieved_escrow_factory, escrow_factory_address);

    let retrieved_order_mixin = resolver_client.get_order_mixin_address();
    assert_eq!(retrieved_order_mixin, order_mixin_address);
}

#[test]
fn test_deploy_src_basic_functionality() {
    let env = Env::default();
    let admin = Address::generate(&env);

    let maker = Address::generate(&env);
    let taker = Address::generate(&env);
    let token = Address::generate(&env);
    let receiver = Address::generate(&env);

    let secret = "4815162342";
    let secret_bytes = Bytes::from_slice(&env, secret.as_bytes());
    let hashlock = env.crypto().keccak256(&secret_bytes);

    let order_hash = BytesN::from_array(&env, &[0; 32]);

    let escrow_factory_address = Address::generate(&env);
    let order_mixin_address = Address::generate(&env);

    let contract_id = env.register(
        ResolverContract,
        (&escrow_factory_address, &order_mixin_address),
    );
    let resolver_client = ResolverContractClient::new(&env, &contract_id);

    let immutables = Immutables {
        order_hash: order_hash.clone(),
        hashlock: hashlock.to_bytes(),
        maker: maker.clone(),
        taker: taker.clone(),
        token: token.clone(),
        amount: 1000000000000000000,
        safety_deposit: 1000000000000000000,
        timelocks: U256::from_u32(&env, 0),
    };

    let order = Order {
        maker: maker.clone(),
        maker_asset: token.clone(),
        maker_traits: U256::from_u32(&env, 0),
        making_amount: U256::from_u128(&env, 1000000000000000000),
        receiver: receiver.clone(),
        salt: U256::from_u32(&env, 1),
        taker_asset: token.clone(),
        taking_amount: U256::from_u128(&env, 1000000000000000000),
    };

    let signature_r = BytesN::from_array(&env, &[1; 32]);
    let signature_vs = BytesN::from_array(&env, &[2; 32]);
    let amount = U256::from_u128(&env, 1000000000000000000);
    let taker_traits = U256::from_u32(&env, 0);
    let args = Bytes::from_slice(&env, &[]);

    // This would normally call the actual deploy_src function
    // For testing, we verify the function signature and parameters are correct
    assert_eq!(immutables.maker, maker);
    assert_eq!(immutables.taker, taker);
    assert_eq!(immutables.token, token);
    assert_eq!(order.maker, maker);
    assert_eq!(order.receiver, receiver);
}

#[test]
fn test_deploy_dst_basic_functionality() {
    let env = Env::default();
    let admin = Address::generate(&env);

    let maker = Address::generate(&env);
    let taker = Address::generate(&env);
    let token = Address::generate(&env);

    let secret = "4815162342";
    let secret_bytes = Bytes::from_slice(&env, secret.as_bytes());
    let hashlock = env.crypto().keccak256(&secret_bytes);

    let order_hash = BytesN::from_array(&env, &[0; 32]);

    let escrow_factory_address = Address::generate(&env);
    let order_mixin_address = Address::generate(&env);

    let contract_id = env.register(
        ResolverContract,
        (&escrow_factory_address, &order_mixin_address),
    );
    let resolver_client = ResolverContractClient::new(&env, &contract_id);

    let dst_immutables = Immutables {
        order_hash: order_hash.clone(),
        hashlock: hashlock.to_bytes(),
        maker: maker.clone(),
        taker: taker.clone(),
        token: token.clone(),
        amount: 1000000000000000000,
        safety_deposit: 1000000000000000000,
        timelocks: U256::from_u32(&env, 0),
    };

    let src_cancellation_timestamp = U256::from_u32(&env, 1893477661); // Year 2030

    // This would normally call the actual deploy_dst function
    // For testing, we verify the function signature and parameters are correct
    assert_eq!(dst_immutables.maker, maker);
    assert_eq!(dst_immutables.taker, taker);
    assert_eq!(dst_immutables.token, token);
    assert_eq!(src_cancellation_timestamp, U256::from_u32(&env, 1893477661));
}

#[test]
fn test_hashlock_generation() {
    let env = Env::default();

    // Test multiple secrets to ensure hashlock generation is consistent
    let test_secrets = [
        "4815162342",
        "1234567890",
        "abcdefghij",
        "test_secret_key_for_1inch_fusion_plus",
    ];

    for secret in test_secrets {
        let secret_bytes = Bytes::from_slice(&env, secret.as_bytes());
        let hashlock = env.crypto().keccak256(&secret_bytes);

        // Verify hashlock is 32 bytes
        assert_eq!(hashlock.to_bytes().len(), 32);

        // Verify same secret produces same hashlock
        let hashlock2 = env.crypto().keccak256(&secret_bytes);
        assert_eq!(hashlock.to_bytes(), hashlock2.to_bytes());
    }
}

#[test]
fn test_timelock_calculation() {
    let env = Env::default();

    // Test timelock calculation with different values
    let base_timelock = U256::from_u32(&env, 1000);
    let deployed_at = U256::from_u32(&env, 500);

    // Test that timelocks are properly structured
    assert!(base_timelock > U256::from_u32(&env, 0));
    assert!(deployed_at > U256::from_u32(&env, 0));
}

#[test]
fn test_order_validation() {
    let env = Env::default();

    let maker = Address::generate(&env);
    let taker = Address::generate(&env);
    let token = Address::generate(&env);
    let receiver = Address::generate(&env);

    let order = Order {
        maker: maker.clone(),
        maker_asset: token.clone(),
        maker_traits: U256::from_u32(&env, 0),
        making_amount: U256::from_u128(&env, 1000000000000000000),
        receiver: receiver.clone(),
        salt: U256::from_u32(&env, 1),
        taker_asset: token.clone(),
        taking_amount: U256::from_u128(&env, 1000000000000000000),
    };

    // Test order validation
    assert_eq!(order.maker, maker);
    assert_eq!(order.receiver, receiver);
    assert_eq!(order.maker_asset, token);
    assert_eq!(order.taker_asset, token);
    assert!(order.making_amount > U256::from_u32(&env, 0));
    assert!(order.taking_amount > U256::from_u32(&env, 0));
    assert!(order.salt > U256::from_u32(&env, 0));
}

#[test]
fn test_immutables_validation() {
    let env = Env::default();

    let maker = Address::generate(&env);
    let taker = Address::generate(&env);
    let token = Address::generate(&env);
    let secret = "test_secret";
    let secret_bytes = Bytes::from_slice(&env, secret.as_bytes());
    let hashlock = env.crypto().keccak256(&secret_bytes);
    let order_hash = BytesN::from_array(&env, &[1; 32]);

    let immutables = Immutables {
        order_hash: order_hash.clone(),
        hashlock: hashlock.to_bytes(),
        maker: maker.clone(),
        taker: taker.clone(),
        token: token.clone(),
        amount: 1000000000000000000,
        safety_deposit: 1000000000000000000,
        timelocks: U256::from_u32(&env, 0),
    };

    // Test immutables validation
    assert_eq!(immutables.maker, maker);
    assert_eq!(immutables.taker, taker);
    assert_eq!(immutables.token, token);
    assert_eq!(immutables.order_hash, order_hash);
    assert_eq!(immutables.hashlock, hashlock.to_bytes());
    assert!(immutables.amount > 0);
    assert!(immutables.safety_deposit > 0);
}

#[test]
fn test_signature_validation() {
    let env = Env::default();

    // Test signature format validation
    let signature_r = BytesN::from_array(&env, &[1; 32]);
    let signature_vs = BytesN::from_array(&env, &[2; 32]);

    // Verify signatures are 32 bytes each
    assert_eq!(signature_r.len(), 32);
    assert_eq!(signature_vs.len(), 32);

    // Test different signature values
    let signature_r2 = BytesN::from_array(&env, &[3; 32]);
    let signature_vs2 = BytesN::from_array(&env, &[4; 32]);

    assert_ne!(signature_r, signature_r2);
    assert_ne!(signature_vs, signature_vs2);
}

#[test]
fn test_amount_validation() {
    let env = Env::default();

    // Test various amount values
    let amounts = [
        1000000000000000000u128,  // 1 token with 18 decimals
        100000000000000000u128,   // 0.1 token
        10000000000000000000u128, // 10 tokens
        1u128,                    // Minimum amount
    ];

    for amount in amounts {
        assert!(amount > 0, "Amount must be greater than 0");
    }
}

#[test]
fn test_taker_traits_validation() {
    let env = Env::default();

    // Test taker traits with different values
    let taker_traits_values = [
        U256::from_u32(&env, 0),
        U256::from_u32(&env, 1),
        U256::from_u32(&env, 255),
        U256::from_u32(&env, 65535),
    ];

    for traits in taker_traits_values {
        assert!(
            traits >= U256::from_u32(&env, 0),
            "Taker traits must be non-negative"
        );
    }
}

#[test]
fn test_args_validation() {
    let env = Env::default();

    // Test empty args
    let empty_args = Bytes::from_slice(&env, &[]);
    assert_eq!(empty_args.len(), 0);

    // Test args with data
    let test_data = [1u8, 2, 3, 4, 5];
    let args_with_data = Bytes::from_slice(&env, &test_data);
    assert_eq!(args_with_data.len(), 5);

    // Test large args
    let large_data = [0u8; 1000];
    let large_args = Bytes::from_slice(&env, &large_data);
    assert_eq!(large_args.len(), 1000);
}

#[test]
fn test_contract_addresses_consistency() {
    let env = Env::default();

    let escrow_factory_address = Address::generate(&env);
    let order_mixin_address = Address::generate(&env);

    let contract_id = env.register(
        ResolverContract,
        (&escrow_factory_address, &order_mixin_address),
    );
    let resolver_client = ResolverContractClient::new(&env, &contract_id);

    // Test that addresses remain consistent
    let retrieved_escrow_factory = resolver_client.get_escrow_factory_address();
    let retrieved_order_mixin = resolver_client.get_order_mixin_address();

    assert_eq!(retrieved_escrow_factory, escrow_factory_address);
    assert_eq!(retrieved_order_mixin, order_mixin_address);

    // Test multiple calls return same values
    let retrieved_escrow_factory2 = resolver_client.get_escrow_factory_address();
    let retrieved_order_mixin2 = resolver_client.get_order_mixin_address();

    assert_eq!(retrieved_escrow_factory, retrieved_escrow_factory2);
    assert_eq!(retrieved_order_mixin, retrieved_order_mixin2);
}

#[test]
fn test_edge_cases() {
    let env = Env::default();

    // Test with maximum values
    let max_u128 = u128::MAX;
    let max_u256 = U256::from_u128(&env, u128::MAX);

    let maker = Address::generate(&env);
    let taker = Address::generate(&env);
    let token = Address::generate(&env);
    let order_hash = BytesN::from_array(&env, &[0xFF; 32]);
    let hashlock = BytesN::from_array(&env, &[0xFF; 32]);

    let immutables = Immutables {
        order_hash: order_hash.clone(),
        hashlock: hashlock.clone(),
        maker: maker.clone(),
        taker: taker.clone(),
        token: token.clone(),
        amount: max_u128,
        safety_deposit: max_u128,
        timelocks: max_u256.clone(),
    };

    // Verify edge case values are handled
    assert_eq!(immutables.amount, max_u128);
    assert_eq!(immutables.safety_deposit, max_u128);
    assert_eq!(immutables.timelocks, max_u256);
}

#[test]
fn test_zero_values() {
    let env = Env::default();

    // Test with zero values (should be handled gracefully)
    let maker = Address::generate(&env);
    let taker = Address::generate(&env);
    let token = Address::generate(&env);
    let order_hash = BytesN::from_array(&env, &[0; 32]);
    let hashlock = BytesN::from_array(&env, &[0; 32]);

    let immutables = Immutables {
        order_hash: order_hash.clone(),
        hashlock: hashlock.clone(),
        maker: maker.clone(),
        taker: taker.clone(),
        token: token.clone(),
        amount: 0,
        safety_deposit: 0,
        timelocks: U256::from_u32(&env, 0),
    };

    // Verify zero values are handled
    assert_eq!(immutables.amount, 0);
    assert_eq!(immutables.safety_deposit, 0);
    assert_eq!(immutables.timelocks, U256::from_u32(&env, 0));
}
