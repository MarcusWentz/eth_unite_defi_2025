#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _,},
    Env, Address, BytesN, U256
};

use crate::{
    Immutables,
    escrow_factory::EscrowFactory,
};

#[test]
fn test_address_of_escrow_src() {
    let env = Env::default();
    let computed_addy = "CALVAFBJBYNRFX5NU6RX46Y6YDRN3YCUXDQTYYMOHJGF3L4H2IOEE4VX";
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