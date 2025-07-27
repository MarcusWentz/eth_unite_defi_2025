#![no_std]
use soroban_sdk::{contracttype, Address, BytesN, U256};

// Data for creating the escrow contracts
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Immutables {
    pub order_hash: BytesN<32>,
    pub hashlock: BytesN<32>,
    pub maker: Address,
    pub taker: Address,
    pub token: Address,
    pub amount: i128,
    pub safety_deposit: U256,
    pub timelocks: U256,
}
