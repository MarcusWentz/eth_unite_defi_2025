#[test]
fn test_dutch_auction_calculator() {
    let env = Env::default();
    let calculator = DutchAuctionCalculator::new(&env);

    let order = Order {
        maker: Address::generate(&env),
        maker_asset: Address::generate(&env),
        taker_asset: Address::generate(&env),
        making_amount: U256::from(100),
        taking_amount: U256::from(100),
        maker_traits: TakerTraits::new(&env),
    };
}