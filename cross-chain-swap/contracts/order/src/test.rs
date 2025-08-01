#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env, U256};

use crate::{Order, OrderProtocol, OrderProtocolClient};
use dutch_auction::DutchAuctionCalculatorContract;
use dutch_auction_interface::AuctionDetails;

#[test]
fn test_calculate_making_amount() {
    let env = Env::default();

    let dutch_auction_calculator_address = env.register(DutchAuctionCalculatorContract, ());

    let contract_id = env.register(OrderProtocol, (&dutch_auction_calculator_address,));
    let _client = OrderProtocolClient::new(&env, &contract_id);

    let _order = Order {
        salt: U256::from_u32(&env, 0),
        maker: Address::generate(&env),
        receiver: Address::generate(&env),
        maker_asset: Address::generate(&env),
        taker_asset: Address::generate(&env),
        making_amount: U256::from_u32(&env, 100),
        taking_amount: U256::from_u32(&env, 50),
        maker_traits: U256::from_u32(&env, 0),
    };

    let _extension = Bytes::from_array(&env, &[0; 0]);
    let requested_taking_amount = U256::from_u32(&env, 100);
    let remaining_making_amount = U256::from_u32(&env, 100);
    let order_hash = BytesN::from_array(&env, &[0; 32]);

    let auction_details = AuctionDetails {
        auction_start_time: U256::from_u32(&env, 1000),
        taking_amount_start: U256::from_u32(&env, 100),
        taking_amount_end: U256::from_u32(&env, 10),
    };

    let res = _client.calculate_making_amount(
        &_order,
        &_extension,
        &requested_taking_amount,
        &remaining_making_amount,
        &order_hash,
        &auction_details,
    );

    assert_eq!(res, U256::from_u32(&env, 100));
}
