#![no_std]
use soroban_sdk::{contractclient, contracttype, Address, Bytes, BytesN, Env, U256};

/// Order structure for cross-chain swaps
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Order {
    /// Salt for order uniqueness
    pub salt: U256,
    /// Maker address
    pub maker: Address,
    /// Receiver address
    pub receiver: Address,
    /// Maker asset
    pub maker_asset: Address,
    /// Taker asset
    pub taker_asset: Address,
    /// Making amount
    pub making_amount: U256,
    /// Taking amount
    pub taking_amount: U256,
    /// Maker traits
    pub maker_traits: U256,
}

#[contracttype]
#[derive(Clone)]
pub struct AuctionDetails {
    pub auction_start_time: U256,
    pub taking_amount_start: U256,
    pub taking_amount_end: U256,
}

#[contractclient(name = "OrderClient")]
pub trait OrderInterface {
    fn __constructor(env: Env, da_addy: Address);

    fn calculate_making_amount(
        env: Env,
        order: Order,
        _extension: Bytes,
        requested_taking_amount: U256,
        remaining_making_amount: U256,
        order_hash: BytesN<32>,
        auction_details: AuctionDetails,
    ) -> U256;

    fn calculate_taking_amount(
        env: Env,
        order: Order,
        _extension: Bytes,
        requested_making_amount: U256,
        remaining_making_amount: U256,
        order_hash: BytesN<32>,
        auction_details: AuctionDetails,
    ) -> U256;

    #[allow(non_snake_case)]
    fn _check_remaining_making_amount(env: Env, order: Order, order_hash: BytesN<32>) -> U256;

    fn order_hash(env: Env, order: Order) -> BytesN<32>;

    fn fill(
        env: Env,
        order: Order,
        order_hash: BytesN<32>,
        remaining_making_amount: U256,
        _amount: U256,
        _taker_traits: U256,
        _target: Address,
        _extension: Bytes,
        _interaction: Bytes,
        auction_details: AuctionDetails,
    );

    fn fill_order(
        env: Env,
        order: Order,
        r: BytesN<32>,
        vs: BytesN<32>,
        amount: U256,
        taker_traits: U256,
        target: Address,
        extension: Bytes,
        interaction: Bytes,
        auction_details: AuctionDetails,
    ) -> (U256, U256, BytesN<32>);

    fn fill_order_args(
        env: Env,
        order: Order,
        r: BytesN<32>,
        vs: BytesN<32>,
        amount: U256,
        taker_traits: U256,
        args: Bytes,
        auction_details: AuctionDetails,
    ) -> (U256, U256, BytesN<32>);
}
