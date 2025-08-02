use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, U256};

use crate::{
    escrow_factory::EscrowFactory, 
    escrow_factory::EscrowFactoryClient,
};

use base_escrow::Immutables;

// The contract that will be deployed by the deployer contract.
mod escrow_dst_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/escrow_dst.wasm"
    );
}

// The contract that will be deployed by the deployer contract.
mod escrow_src_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/escrow_src.wasm"
    );
}

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
fn test_create_dst_escrow() {

    let env = Env::default();    

    let escrow_dst_wasm_hash = env.deployer().upload_contract_wasm(escrow_dst_contract::WASM);
    let escrow_src_wasm_hash = env.deployer().upload_contract_wasm(escrow_src_contract::WASM);

    let xlm_address = Address::from_str(&env, "CCJNI7JJQF23TO3PVBIN3V4R66EWBD3AFNQ6EL4POPSXHZT4IYXIQ5KI");

    let contract_id = env.register(EscrowFactory, (escrow_dst_wasm_hash, escrow_src_wasm_hash, xlm_address));
    let client = EscrowFactoryClient::new(&env, &contract_id);
    // 1893477661 unix time is the start of year 2030
    let input_src_cancellation_timestamp : U256 = U256::from_u32(&env, 1893477661);
    
    //Define immutables struct values.
    let address_input_test : Address = Address::from_str(&env, "CCJNI7JJQF23TO3PVBIN3V4R66EWBD3AFNQ6EL4POPSXHZT4IYXIQ5KI");
    let u256_input_test : U256 = U256::from_u32(&env, 1);
    let u128_input_test : u128 = 1;
    let raw_value_max_uint256_bytes = [0xFFu8; 32]; // 32 bytes of 0xFF
    let bytes_32_max: BytesN<32> = BytesN::from_array(&env, &raw_value_max_uint256_bytes);
    let input_immutables = Immutables { 
        order_hash: bytes_32_max.clone(), 
        hashlock: bytes_32_max.clone(), 
        maker: address_input_test.clone(), 
        taker: address_input_test.clone(), 
        token: address_input_test.clone(), 
        amount: u128_input_test, 
        safety_deposit: u128_input_test, 
        timelocks: u256_input_test 
    };

    env.mock_all_auths();
    
    // // Test create_dst_escrow with return address from function call.
    let test_address_return_output : Address = client.create_dst_escrow(
        &input_immutables.clone(), 
        &input_src_cancellation_timestamp.clone(),
        &u128_input_test
    );

    let output_address : Address = Address::from_str(&env, "CBOYRJDYA5LM652UWKZGSSDRJNJYE76URGF4B7HQ3LY5EFWRR3VVENSF");
    assert_eq!(test_address_return_output, output_address);

}
