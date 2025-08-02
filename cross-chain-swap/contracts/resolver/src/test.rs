#![cfg(test)]

use crate::{ResolverContract, ResolverContractClient};
use base_escrow::Immutables;
use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env, U256};

#[test]
fn test() {
    let env = Env::default();
    let admin = Address::generate(&env);

    let escrow_factory_address = Address::generate(&env);
    let order_mixin_address = Address::generate(&env);

    let contract_id = env.register(
        ResolverContract,
        (&escrow_factory_address, &order_mixin_address),
    );
    let resolver_client = ResolverContractClient::new(&env, &contract_id);

    // Invoke contract to check that it is initialized.
    let escrow_factory_address = resolver_client.get_escrow_factory_address();
    assert_eq!(escrow_factory_address, escrow_factory_address);

    let order_mixin_address = resolver_client.get_order_mixin_address();
    assert_eq!(order_mixin_address, order_mixin_address);
}

#[test]
fn test_deploy_src() {
    let secret = "4815162342";

    let env = Env::default();
    let admin = Address::generate(&env);

    let maker = Address::generate(&env);
    let taker = Address::generate(&env);
    let token = Address::generate(&env);

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
        order_hash: BytesN::from_array(&env, &[0; 32]),
        hashlock: BytesN::from_array(&env, &[0; 32]),
        maker: maker,
        taker: taker,
        token: token,
        amount: 1000000000000000000,
        safety_deposit: 1000000000000000000,
        timelocks: U256::from_u32(&env, 0), // would be set inside of deploy_src
    };
}
