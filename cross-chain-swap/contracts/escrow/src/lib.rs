#![no_std]
use soroban_sdk::{contracttype, Address, BytesN, U256};

// Data for creating the escrow contracts
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
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

pub mod escrow_factory;
pub mod timelocks;

#[cfg(test)]
mod test_escrow_factory;
