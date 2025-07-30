#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(OrderProtocol, ());
    let client = OrderProtocolClient::new(&env, &contract_id);

    let order = Order {
        salt: U256::from_u32(&env, 0),
        maker: Address::generate(&env),
        receiver: Address::generate(&env),
        maker_asset: Address::generate(&env),
        taker_asset: Address::generate(&env),
        making_amount: U256::from_u32(&env, 0),
        taking_amount: U256::from_u32(&env, 0),
        maker_traits: U256::from_u32(&env, 0),
    };
}
