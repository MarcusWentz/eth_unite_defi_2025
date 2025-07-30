#![no_std]
use soroban_sdk::{contracttype, contract, contractimpl, Address, BytesN, U256};

use order::Order;

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
    pub safety_deposit: i128,
    pub timelocks: U256,
}

pub mod timelocks;
pub mod escrow_factory;

#[cfg(test)]
mod test_escrow_factory;