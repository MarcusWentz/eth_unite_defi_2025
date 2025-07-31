#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _,}, 
    Env, Address, U256
};
use crate::{DutchAuctionCalculatorContract, DutchAuctionCalculatorContractClient};

#[test]
fn test_dutch_auction_calculator() {
    let env = Env::default();
    let contract_id = env.register(DutchAuctionCalculatorContract, ());
    let client = DutchAuctionCalculatorContractClient::new(&env, &contract_id);

    let auction_details = AuctionDetails {
        auction_start_time: U256::from_u128(&env, 1000),
        taking_amount_start: U256::from_u128(&env, 100),
        taking_amount_end: U256::from_u128(&env, 10),
    };

    // let making_amount = client.get_making_amount(&env, &auction_details);

    // let order = Order {
    //     maker: Address::generate(&env),
    //     maker_asset: Address::generate(&env),
    //     taker_asset: Address::generate(&env),
    //     making_amount: U256::from_u128(&env, 100),
    //     taking_amount: U256::from_u128(&env, 100),
    //     maker_traits: MakerTraitsBuilder::new(env.clone()).build(),
    //     receiver: Address::from_str(&env, "0x0000000000000000000000000000000000000000000000000000000000000000"),
    //     salt: U256::from_u128(&env, 1),
    // };


}