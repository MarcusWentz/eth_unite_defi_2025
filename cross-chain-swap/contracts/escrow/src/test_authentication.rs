#![cfg(test)]

use crate::escrow_factory::{EscrowFactory, EscrowFactoryClient};
use base_escrow::Immutables;
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, U256};

#[test]
#[should_panic(expected = "AccessDenied")]
fn test_create_dst_escrow_unauthorized_maker() {
    let env = Env::default();
    
    // Create test addresses
    let authorized_maker = Address::generate(&env);
    let unauthorized_maker = Address::generate(&env);
    let taker = Address::generate(&env);
    let token = Address::generate(&env);
    
    // Upload contract WASMs
    let escrow_dst_wasm_hash = env
        .deployer()
        .upload_contract_wasm(include_bytes!("../../../escrow-dst/target/wasm32-unknown-unknown/release/escrow_dst.wasm"));
    let escrow_src_wasm_hash = env
        .deployer()
        .upload_contract_wasm(include_bytes!("../../../escrow-src/target/wasm32-unknown-unknown/release/escrow_src.wasm"));
    
    let xlm_address = Address::from_str(
        &env,
        "CCJNI7JJQF23TO3PVBIN3V4R66EWBD3AFNQ6EL4POPSXHZT4IYXIQ5KI",
    );
    
    // Deploy factory contract
    let contract_id = env.register(
        EscrowFactory,
        (escrow_dst_wasm_hash, escrow_src_wasm_hash, xlm_address),
    );
    let client = EscrowFactoryClient::new(&env, &contract_id);
    
    // Create test immutables
    let immutables = Immutables {
        order_hash: BytesN::from_array(&env, &[1u8; 32]),
        hashlock: BytesN::from_array(&env, &[2u8; 32]),
        maker: authorized_maker.clone(),
        taker: taker.clone(),
        token: token.clone(),
        amount: 1000000000000000000u128,
        safety_deposit: 100000000000000000u128,
        timelocks: U256::from_u32(&env, 1000),
    };
    
    let src_cancellation_timestamp = U256::from_u32(&env, 2000);
    let native_token_lock_value = 100000000000000000u128;
    
    // Set unauthorized maker as sender (this should fail)
    env.storage()
        .persistent()
        .set(&soroban_sdk::symbol_short!("sender"), &unauthorized_maker);
    
    // This should panic with "AccessDenied" because unauthorized_maker is not the maker
    let _ = client.create_dst_escrow(
        &immutables,
        &src_cancellation_timestamp,
        &native_token_lock_value,
    );
}

#[test]
fn test_create_dst_escrow_authorized_maker() {
    let env = Env::default();
    
    // Create test addresses
    let authorized_maker = Address::generate(&env);
    let taker = Address::generate(&env);
    let token = Address::generate(&env);
    
    // Upload contract WASMs
    let escrow_dst_wasm_hash = env
        .deployer()
        .upload_contract_wasm(include_bytes!("../../../escrow-dst/target/wasm32-unknown-unknown/release/escrow_dst.wasm"));
    let escrow_src_wasm_hash = env
        .deployer()
        .upload_contract_wasm(include_bytes!("../../../escrow-src/target/wasm32-unknown-unknown/release/escrow_src.wasm"));
    
    let xlm_address = Address::from_str(
        &env,
        "CCJNI7JJQF23TO3PVBIN3V4R66EWBD3AFNQ6EL4POPSXHZT4IYXIQ5KI",
    );
    
    // Deploy factory contract
    let contract_id = env.register(
        EscrowFactory,
        (escrow_dst_wasm_hash, escrow_src_wasm_hash, xlm_address),
    );
    let client = EscrowFactoryClient::new(&env, &contract_id);
    
    // Create test immutables
    let immutables = Immutables {
        order_hash: BytesN::from_array(&env, &[1u8; 32]),
        hashlock: BytesN::from_array(&env, &[2u8; 32]),
        maker: authorized_maker.clone(),
        taker: taker.clone(),
        token: token.clone(),
        amount: 1000000000000000000u128,
        safety_deposit: 100000000000000000u128,
        timelocks: U256::from_u32(&env, 1000),
    };
    
    let src_cancellation_timestamp = U256::from_u32(&env, 2000);
    let native_token_lock_value = 100000000000000000u128;
    
    // Set authorized maker as sender
    env.storage()
        .persistent()
        .set(&soroban_sdk::symbol_short!("sender"), &authorized_maker);
    
    // Mock all auths for testing
    env.mock_all_auths();
    
    // This should succeed because authorized_maker is the maker
    let result = client.create_dst_escrow(
        &immutables,
        &src_cancellation_timestamp,
        &native_token_lock_value,
    );
    
    // Verify the result is a valid address
    assert_ne!(result, Address::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"));
} 