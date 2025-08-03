#![cfg(test)]

use super::*;
use crate::{DutchAuctionCalculatorContract, DutchAuctionCalculatorContractClient};
use order_interface::{AuctionDetails, Order};
use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env, U256};

#[test]
fn test_dutch_auction_calculator_get_making_amount() {
    let env = Env::default();
    let contract_id = env.register(DutchAuctionCalculatorContract, ());
    let client = DutchAuctionCalculatorContractClient::new(&env, &contract_id);

    let auction_details = AuctionDetails {
        auction_start_time: U256::from_u128(&env, 1000),
        taking_amount_start: U256::from_u128(&env, 100),
        taking_amount_end: U256::from_u128(&env, 10),
    };

    let order = Order {
        maker: Address::generate(&env),
        maker_asset: Address::generate(&env),
        taker_asset: Address::generate(&env),
        making_amount: U256::from_u128(&env, 100),
        taking_amount: U256::from_u128(&env, 1),
        maker_traits: U256::from_u128(&env, 0),
        receiver: Address::generate(&env),
        salt: U256::from_u128(&env, 0),
    };

    let res = client.get_making_amount(
        &order,
        &Bytes::from_array(&env, &[0; 0]),
        &BytesN::from_array(&env, &[0; 32]),
        &Address::generate(&env),
        &U256::from_u128(&env, 100),
        &U256::from_u128(&env, 100),
        &auction_details,
    );

    assert_eq!(res, U256::from_u128(&env, 100));
}

#[test]
fn test_dutch_auction_calculator_get_taking_amount() {
    let env = Env::default();
    let contract_id = env.register(DutchAuctionCalculatorContract, ());
    let client = DutchAuctionCalculatorContractClient::new(&env, &contract_id);

    let auction_details = AuctionDetails {
        auction_start_time: U256::from_u128(&env, 1000),
        taking_amount_start: U256::from_u128(&env, 100),
        taking_amount_end: U256::from_u128(&env, 10),
    };

    let order = Order {
        maker: Address::generate(&env),
        maker_asset: Address::generate(&env),
        taker_asset: Address::generate(&env),
        making_amount: U256::from_u128(&env, 100),
        taking_amount: U256::from_u128(&env, 1),
        maker_traits: U256::from_u128(&env, 0),
        receiver: Address::generate(&env),
        salt: U256::from_u128(&env, 0),
    };

    let res = client.get_taking_amount(
        &order,
        &Bytes::from_array(&env, &[0; 0]),
        &BytesN::from_array(&env, &[0; 32]),
        &Address::generate(&env),
        &U256::from_u128(&env, 100),
        &U256::from_u128(&env, 100),
        &auction_details,
    );

    assert_eq!(res, U256::from_u128(&env, 100));
}

#[test]
fn test_dutch_auction_calculator_calculate_auction_taking_amount() {
    let env = Env::default();
    let contract_id = env.register(DutchAuctionCalculatorContract, ());
    let client = DutchAuctionCalculatorContractClient::new(&env, &contract_id);

    let res = client.calculate_auction_taking_amount(
        &U256::from_u128(&env, 1000),
        &U256::from_u128(&env, 100),
        &U256::from_u128(&env, 1100),
    );

    assert_eq!(res, U256::from_u128(&env, 100));
}

#[test]
fn test_bit_and() {
    let env = Env::default();
    let a = U256::from_u128(&env, 0x1234567890abcdef);
    let b = U256::from_u128(&env, 0xabcdef1234567890);
    let res = bitand(&env, a, b);
    assert_eq!(
        res.to_be_bytes(),
        U256::from_u128(&env, 0x0204461010024880).to_be_bytes()
    );
}

#[test]
fn test_bit_and_meaningful_test() {
    let env = Env::default();
    let a = U256::from_u128(&env, 10000123);
    let b = U256::from_u128(&env, 10000120);
    let res = bitand(&env, a, b);
    assert_eq!(res, U256::from_u128(&env, 10000120));
}
