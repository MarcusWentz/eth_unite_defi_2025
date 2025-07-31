use soroban_sdk::{
    Env, Address, U256,
    testutils::Address as TestAddress,
};

use crate::dutch_auction::{DutchAuctionCalculator, AuctionDetails};
use crate::maker_traits::{MakerTraitsLib, MakerTraitsBuilder};

#[test]
fn test_dutch_auction_calculator() {
    let env = Env::default();
    let calculator = DutchAuctionCalculator::new(&env);

    let maker_traits = 255 - 1;

    let order = Order {
        maker: Address::generate(&env),
        maker_asset: Address::generate(&env),
        taker_asset: Address::generate(&env),
        making_amount: U256::from_u128(&env, 100),
        taking_amount: U256::from_u128(&env, 100),
        maker_traits: MakerTraitsBuilder::new(env.clone()).build(),
        receiver: Address::from_str(&env, "0x0000000000000000000000000000000000000000000000000000000000000000"),
        salt: U256::from_u128(&env, 1),
    };


}