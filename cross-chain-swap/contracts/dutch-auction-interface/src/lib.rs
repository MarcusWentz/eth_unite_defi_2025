#![no_std]
use order_interface::Order;
use soroban_sdk::{contractclient, contracttype, Address, Bytes, BytesN, Env, U256};

#[contracttype]
#[derive(Clone)]
pub struct AuctionDetails {
    pub auction_start_time: U256,
    pub taking_amount_start: U256,
    pub taking_amount_end: U256,
}

#[contractclient(name = "DutchAuctionCalculatorContractClient")]
pub trait DutchAuctionCalculatorInterface {
    fn get_making_amount(
        env: Env,
        order: Order,
        extension: Bytes,
        order_hash: BytesN<32>,
        taker: Address,
        taking_amount: U256,
        remaining_making_amount: U256,
        auction_details: AuctionDetails,
    ) -> U256;

    fn get_taking_amount(
        env: Env,
        order: Order,
        extension: Bytes,
        order_hash: BytesN<32>,
        taker: Address,
        making_amount: U256,
        remaining_making_amount: U256,
        auction_details: AuctionDetails,
    ) -> U256;

    fn calculate_auction_taking_amount(
        env: Env,
        auction_start_time: U256,
        taking_amount_start: U256,
        taking_amount_end: U256,
    ) -> U256;
}
