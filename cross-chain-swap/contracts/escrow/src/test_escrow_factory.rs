#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, U256};

use crate::{
    escrow_factory::EscrowFactory, 
    escrow_factory::EscrowType, 
    escrow_factory::EscrowFactoryClient,
    Immutables};

#[test]
fn test_address_of_escrow_src() {
    let env = Env::default();
    let computed_addy = "CCJNI7JJQF23TO3PVBIN3V4R66EWBD3AFNQ6EL4POPSXHZT4IYXIQ5KI";
    let pre_computed_address = Address::from_str(&env, computed_addy);
    let immutables = Immutables {
        order_hash: BytesN::from_array(&env, &[0; 32]),
        hashlock: BytesN::from_array(&env, &[0; 32]),
        maker: Address::generate(&env),
        taker: Address::generate(&env),
        token: Address::generate(&env),
        amount: 1000000000000000000,
        safety_deposit: 1000000000000000000,
        timelocks: U256::from_u32(&env, 0),
    };

    let address = EscrowFactory::address_of_escrow_src(&env, immutables);
    assert_eq!(address, pre_computed_address);
}


#[test]
fn test_store_escrow_wasm_escrowType_source() {

    let env = Env::default();    
    let contract_id = env.register(EscrowFactory, ());
    let client = EscrowFactoryClient::new(&env, &contract_id);

    let escrow_type_status: EscrowType = EscrowType::Source;
    let raw_value_max_uint256_bytes = [0xFFu8; 32]; // 32 bytes of 0xFF
    let bytes_32_max: BytesN<32> = BytesN::from_array(&env, &raw_value_max_uint256_bytes);
    
    // Test source path.
    client.store_escrow_wasm(&bytes_32_max, &escrow_type_status);
}

#[test]
fn test_store_escrow_wasm_escrowType_destination() {

    let env = Env::default();    
    let contract_id = env.register(EscrowFactory, ());
    let client = EscrowFactoryClient::new(&env, &contract_id);

    let escrow_type_status: EscrowType = EscrowType::Destination;
    let raw_value_max_uint256_bytes = [0xFFu8; 32]; // 32 bytes of 0xFF
    let bytes_32_max: BytesN<32> = BytesN::from_array(&env, &raw_value_max_uint256_bytes);
    
    // Test destination path.
    client.store_escrow_wasm(&bytes_32_max, &escrow_type_status);
}

