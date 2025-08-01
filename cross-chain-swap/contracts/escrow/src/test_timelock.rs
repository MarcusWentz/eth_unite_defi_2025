#![cfg(test)]

// use super::*;
use crate::timelocks::{Stage, Timelocks, TimelocksClient};
use soroban_sdk::{Bytes, Env, U256};

#[test]
fn test_timelock_get_return_1() {
    let env = Env::default();
    let contract_id = env.register(Timelocks, ());
    let client = TimelocksClient::new(&env, &contract_id);

    let stage_status: Stage = Stage::SrcWithdrawal;
    let timelock_input_u256: U256 = U256::from_u32(&env, 1);

    let test_return_value_256: U256 = client.get(&timelock_input_u256, &stage_status);
    // // let test_return_value_256: U256 = get(&timelock_input_u256, &stage_status);

    // Expect to return 1 as type uin256.
    assert_eq!(test_return_value_256, U256::from_u32(&env, 1));
}

#[test]
fn test_timelock_set_deployed_at_mask_value_0() {
    let env = Env::default();
    let contract_id = env.register(Timelocks, ());
    let client = TimelocksClient::new(&env, &contract_id);

    let timelock_input_u256: U256 = U256::from_u32(&env, 100);
    let mask_value_input_u256: U256 = U256::from_u32(&env, 0);

    let test_return_value_256: U256 =
        client.set_deployed_at(&timelock_input_u256, &mask_value_input_u256);

    // Expect to return 100 as type uint256.
    assert_eq!(test_return_value_256, U256::from_u32(&env, 100));
}

#[test]
fn test_timelock_set_deployed_at_mask_value_1() {
    let env = Env::default();
    let contract_id = env.register(Timelocks, ());
    let client = TimelocksClient::new(&env, &contract_id);

    let timelock_input_u256: U256 = U256::from_u32(&env, 1);
    let mask_value_input_u256: U256 = U256::from_u32(&env, 1);

    let test_return_value_256: U256 =
        client.set_deployed_at(&timelock_input_u256, &mask_value_input_u256);

    let test_return_value_bytes: Bytes = test_return_value_256.to_be_bytes();

    // Create a new Bytes array which we can modify.
    let mut solidity_output_bytes_array: Bytes = Bytes::new(&env);
    // // Modify Bytes array to be uin256 max value.
    // bytes_array.extend_from_array(&[0xFFu8; 32]);

    // Modify bytes array to match output.
    // In decimal format:
    // set_deployed_at(1,1) from the 1inch contract in Solidity is:
    // 26959946667150639794667015087019630673637144422540572481103610249217
    solidity_output_bytes_array.extend_from_array(&[
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    ]);

    // Expect to return 26959946667150639794667015087019630673637144422540572481103610249217 as type bytes.
    assert_eq!(test_return_value_bytes, solidity_output_bytes_array);
}

#[test]
fn test_timelock_set_deployed_at_timelock_gt_mask() {
    let env = Env::default();
    let contract_id = env.register(Timelocks, ());
    let client = TimelocksClient::new(&env, &contract_id);

    // Create a new Bytes array which we can modify.
    let mut timelock_input_bytes_array: Bytes = Bytes::new(&env);

    // // 115792089237316195423570985008687907853269984665640564039457584007913129639935
    // // Maximum uint256 value (32 bytes of all 0xFF)
    // bytes_array.extend_from_array(&[
    //     0xFF, 0xFF, 0xFF, 0xFF,
    //     0xFF, 0xFF, 0xFF, 0xFF,
    //     0xFF, 0xFF, 0xFF, 0xFF,
    //     0xFF, 0xFF, 0xFF, 0xFF,
    //     0xFF, 0xFF, 0xFF, 0xFF,
    //     0xFF, 0xFF, 0xFF, 0xFF,
    //     0xFF, 0xFF, 0xFF, 0xFF,
    //     0xFF, 0xFF, 0xFF, 0xFF,
    // ]);

    // 26959946667150639794667015087019630673637144422540572481103610249217
    timelock_input_bytes_array.extend_from_array(&[
        0x03, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0xff, 0xff, 0xfc, 0x2f, 0x00,
        0x00, 0x00,
    ]);

    let timelock_input_u256: U256 = U256::from_be_bytes(&env, &timelock_input_bytes_array);
    let mask_value_input_u256: U256 = U256::from_u32(&env, 1);

    let test_return_value_256: U256 =
        client.set_deployed_at(&timelock_input_u256, &mask_value_input_u256);

    let test_return_value_bytes: Bytes = test_return_value_256.to_be_bytes();

    // Create a new Bytes array which we can modify.
    let mut solidity_output_bytes_array: Bytes = Bytes::new(&env);

    // // Modify Bytes array to be uin256 max value.
    // bytes_array.extend_from_array(&[0xFFu8; 32]);

    // Modify bytes array to match output.
    // In decimal format:
    // set_deployed_at(1,1) from the 1inch contract in Solidity is:

    solidity_output_bytes_array.extend_from_array(&[
        0x00, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0xff, 0xff,
        0xfc, 0x2f,
    ]);

    // // Expect to return 100 as type uint256.
    // assert_eq!(test_return_value_bytes, timelock_input_bytes_array);
}

#[test]
fn test_timelock_rescue_start() {
    let env = Env::default();
    let contract_id = env.register(Timelocks, ());
    let client = TimelocksClient::new(&env, &contract_id);

    let timelock_input_u256: U256 = U256::from_u32(&env, 1);
    let rescue_delay_u256: U256 = U256::from_u32(&env, 1);

    let test_return_value_256: U256 = client.rescue_start(&timelock_input_u256, &rescue_delay_u256);

    assert_eq!(test_return_value_256, U256::from_u32(&env, 1));
}
