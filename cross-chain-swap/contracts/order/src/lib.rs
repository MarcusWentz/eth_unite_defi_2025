#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Address, Env, String, Vec, U256};
pub mod consts_trait;
pub mod maker_traits;
pub mod taker_traits;
pub mod dutch_auction;

#[contract]
struct OrderProtocol {

}

#[contractimpl]
impl OrderProtocol {

    fn calculate_making_amount(
        env: &Env,
        making_amount: U256, // Math.min(amount, remainingMakingAmount)
    ) -> U256 {

    }

    fn fill(
        env: &Env,
        order: Order,
        orderHash: BytesN<32>,
        remainingMakingAmount: U256,
        amount: U256,
        taker_traits: U256,
        target: Address,
        _extension: Bytes,
        _interaction: Bytes,
    ) {
        // ignoring _extension validation phase.

        if !order.maker_traits.is_allowed_sender(target) {
            panic!("Private order");
        }

        if order.maker_traits.is_expired() {
            panic!("Order expired");
        }

        if order.maker_traits.need_check_epoch_manager() {
            if order.maker_traits.use_bit_invalidator() {
                panic!("Epoch manager and bit invalidators are incompatible");
            }
            // todo: @Skanislav implement check:
            // if (!epochEquals(order.maker.get(), order.makerTraits.series(), order.makerTraits.nonceOrEpoch())) revert WrongSeriesNonce();
        }

        // ignoring extension predicate check.

        // Checks if the taking amount should be calculated based on making amount.
        let is_making_amount = true; // takerTraits.isMakingAmount
        if (is_making_amount) {
            let taking_amount = Self::calculate_taking_amount(
                extension,
                making_amount,
                remaining_making_amount,
                order_hash,
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
