#![no_std]
use soroban_sdk::{contracttype, Address, U256};

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
