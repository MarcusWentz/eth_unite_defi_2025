#![no_std]
use soroban_sdk::{contracttype, vec, Env, String, Vec, U256, Address};

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

/*

/// Specifies threshold as maximum allowed takingAmount when takingAmount is zero, otherwise specifies
// * minimum allowed makingAmount. The 2nd (0 based index) highest bit specifies whether taker wants to skip maker's permit.
// #[contracttype]
// pub type TakerTraits = U256;
*/