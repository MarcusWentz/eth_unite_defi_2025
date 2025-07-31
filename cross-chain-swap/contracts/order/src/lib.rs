#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, BytesN, Env, U256};
use crate::maker_traits::MakerTraitsLib;
pub mod consts_trait;
pub mod maker_traits;
pub mod taker_traits;

#[contract]
pub struct OrderProtocol;

#[contractimpl]
impl OrderProtocol {

    fn calculate_making_amount(
        env: Env,
        _making_amount: U256, // Math.min(amount, remainingMakingAmount)
    ) -> U256 {
        return U256::from_u32(&env, 0);
    }

    fn fill(
        env: Env,
        order: Order,
        _order_hash: BytesN<32>,
        remaining_making_amount: U256,
        _amount: U256,
        _taker_traits: U256,
        _target: Address,
        _extension: Bytes,
        _interaction: Bytes,
    ) {
        // ignoring _extension validation phase.

        if !MakerTraitsLib::is_allowed_sender(&env, order.maker_traits.clone(), _target) {
            panic!("Private order");
        }

        if MakerTraitsLib::is_expired(&env, order.maker_traits.clone()) {
            panic!("Order expired");
        }

        if MakerTraitsLib::need_check_epoch_manager(&env, order.maker_traits.clone()) {
            if MakerTraitsLib::use_bit_invalidator(&env, order.maker_traits.clone()) {
                panic!("Epoch manager and bit invalidators are incompatible");
            }
            // todo: @Skanislav implement check:
            // if (!epochEquals(order.maker.get(), order.makerTraits.series(), order.makerTraits.nonceOrEpoch())) revert WrongSeriesNonce();
        }

        // ignoring extension predicate check.

        // Checks if the taking amount should be calculated based on making amount.
        let is_making_amount = true; // takerTraits.isMakingAmount
        if is_making_amount {
            let _taking_amount = Self::calculate_making_amount(
                env,
                U256::min(order.making_amount, remaining_making_amount),
            );
        };


    }
}

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

mod test;
