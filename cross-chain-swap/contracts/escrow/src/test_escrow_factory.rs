#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, U256};

use crate::{escrow_factory::EscrowFactory, Immutables};

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
