#![cfg(test)]

use crate::resolver::{resolver};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    IntoVal, Address, BytesN, Env, Val, Vec
};

#[test]
fn test() {
    let env = Env::default();
    let admin = Address::generate(&env);

    let escrow_factory_address = Address::generate(&env);
    let order_mixin_address = Address::generate(&env);

    let contract_id = env.register(resolver::ResolverContract, (&escrow_factory_address, &order_mixin_address));
    let resolver_client = resolver::ResolverContractClient::new(&env, &contract_id);

    // Invoke contract to check that it is initialized.
    let escrow_factory_address = resolver_client.get_escrow_factory_address();
    assert_eq!(escrow_factory_address, escrow_factory_address);

    let order_mixin_address = resolver_client.get_order_mixin_address();
    assert_eq!(order_mixin_address, order_mixin_address);



}