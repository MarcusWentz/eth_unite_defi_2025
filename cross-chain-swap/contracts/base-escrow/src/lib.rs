#![no_std]
use soroban_sdk::{contracttype, Address, BytesN, U256};

pub mod base_escrow;
pub mod timelocks;

// Data for creating the escrow contracts
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Immutables {
    pub order_hash: BytesN<32>,
    pub hashlock: BytesN<32>,
    pub maker: Address,
    pub taker: Address,
    pub token: Address,
    pub amount: u128,
    pub safety_deposit: u128,
    pub timelocks: U256,
}

mod test_timelock;
