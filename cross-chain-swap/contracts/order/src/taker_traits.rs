use soroban_sdk::{Env, U256};
use utils::math::bitand;

use crate::consts_trait::{u256_bitwise_and, ConstTrait};

// 1inch Solidity version:
// https://github.com/1inch/limit-order-protocol/blob/master/contracts/libraries/TakerTraitsLib.sol

/// TakerTraitsLib equivalent for Soroban
///
/// The TakerTraits type is a U256 and different parts of the number are used to encode different traits.
/// High bits are used for flags:
/// 255 bit `_MAKER_AMOUNT_FLAG`           - If set, the taking amount is calculated based on making amount, otherwise making amount is calculated based on taking amount.
/// 254 bit `_UNWRAP_WETH_FLAG`            - If set, the WETH will be unwrapped into ETH before sending to taker.
/// 253 bit `_SKIP_ORDER_PERMIT_FLAG`      - If set, the order skips maker's permit execution.
/// 252 bit `_USE_PERMIT2_FLAG`            - If set, the order uses the permit2 function for authorization.
/// 251 bit `_ARGS_HAS_TARGET`             - If set, then first 20 bytes of args are treated as target address for maker's funds transfer.
/// 224-247 bits `ARGS_EXTENSION_LENGTH`   - The length of the extension calldata in the args.
/// 200-223 bits `ARGS_INTERACTION_LENGTH` - The length of the interaction calldata in the args.
/// 0-184 bits                             - The threshold amount (the maximum amount a taker agrees to give in exchange for a making amount).
///

pub struct TakerTraitsLib;

impl ConstTrait for TakerTraitsLib {}

impl TakerTraitsLib {
    /// Checks if the args should contain target address.
    fn args_has_target(env: &Env, taker_traits: U256) -> bool {
        u256_bitwise_and(
            &env,
            &taker_traits,
            &Self::args_has_target_const(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Retrieves the length of the extension calldata from the takerTraits.
    fn args_extension_length(env: &Env, taker_traits: U256) -> U256 {
        u256_bitwise_and(
            &env,
            &taker_traits.shr(Self::ARGS_EXTENSION_LENGTH_OFFSET),
            &U256::from_u32(&env, Self::ARGS_EXTENSION_LENGTH_MASK),
        )
    }

    /// Retrieves the length of the interaction calldata from the takerTraits.
    fn args_interaction_length(env: &Env, taker_traits: U256) -> U256 {
        u256_bitwise_and(
            &env,
            &taker_traits.shr(Self::ARGS_INTERACTION_LENGTH_OFFSET),
            &U256::from_u32(&env, Self::ARGS_INTERACTION_LENGTH_MASK),
        )
    }

    /// Checks if the taking amount should be calculated based on making amount.
    pub fn is_making_amount(env: &Env, taker_traits: &U256) -> bool {
        u256_bitwise_and(&env, &taker_traits, &Self::maker_amount_flag(env.clone()))
            .ne(&U256::from_u32(&env, 0))
    }

    /// Checks if the order should unwrap WETH and send ETH to taker.
    pub fn unwrap_weth(env: &Env, taker_traits: U256) -> bool {
        u256_bitwise_and(
            &env,
            &taker_traits,
            &Self::unwrap_weth_taker_flag(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Checks if the order should skip maker's permit execution.
    fn skip_maker_permit(env: &Env, taker_traits: U256) -> bool {
        u256_bitwise_and(
            &env,
            &taker_traits,
            &Self::skip_order_permit_flag(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Checks if the order uses the permit2 instead of permit.
    fn use_permit2(env: &Env, taker_traits: U256) -> bool {
        u256_bitwise_and(
            &env,
            &taker_traits,
            &Self::use_permit2_taker_flag(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Retrieves the threshold amount from the takerTraits.
    /// The maximum amount a taker agrees to give in exchange for a making amount.
    pub fn threshold(env: &Env, taker_traits: U256) -> U256 {
        bitand(&env, taker_traits, Self::amount_mask(env))
    }
}

// Builder pattern for constructing TakerTraits
pub struct TakerTraitsBuilder {
    traits: U256,
    env: Env,
}

impl TakerTraitsBuilder {
    pub fn new(env: Env) -> Self {
        Self {
            traits: U256::from_u32(&env, 0),
            env,
        }
    }

    pub fn with_allowed_sender(mut self, sender_bits: u128) -> Self {
        let sender_u256 = U256::from_u128(&self.env, sender_bits);
        let mask = TakerTraitsLib::allowed_sender_mask(self.env.clone());
        let masked_sender = bitand(&self.env, sender_u256, mask);
        self.traits = self.traits.add(&masked_sender);
        self
    }

    pub fn with_expiration(mut self, expiration: u64) -> Self {
        let expiration_u256 = U256::from_u128(&self.env, expiration as u128);
        let mask = TakerTraitsLib::expiration_mask(self.env.clone());
        let masked_expiration = bitand(&self.env, expiration_u256, mask);
        let shifted = masked_expiration.shl(TakerTraitsLib::EXPIRATION_OFFSET);
        self.traits = self.traits.add(&shifted);
        self
    }

    pub fn with_nonce_or_epoch(mut self, nonce_or_epoch: u64) -> Self {
        let nonce_u256 = U256::from_u128(&self.env, nonce_or_epoch as u128);
        let mask = TakerTraitsLib::nonce_or_epoch_mask(self.env.clone());
        let masked_nonce = bitand(&self.env, nonce_u256, mask);
        let shifted = masked_nonce.shl(TakerTraitsLib::NONCE_OR_EPOCH_OFFSET);
        self.traits = self.traits.add(&shifted);
        self
    }

    pub fn with_series(mut self, series: u64) -> Self {
        let series_u256 = U256::from_u128(&self.env, series as u128);
        let mask = TakerTraitsLib::series_mask(self.env.clone());
        let masked_series = bitand(&self.env, series_u256, mask);
        let shifted = masked_series.shl(TakerTraitsLib::SERIES_OFFSET);
        self.traits = self.traits.add(&shifted);
        self
    }

    pub fn no_partial_fills(mut self) -> Self {
        let flag = TakerTraitsLib::no_partial_fills_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn allow_multiple_fills(mut self) -> Self {
        let flag = TakerTraitsLib::allow_multiple_fills_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn with_pre_interaction_call(mut self) -> Self {
        let flag = TakerTraitsLib::pre_interaction_call_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn with_post_interaction_call(mut self) -> Self {
        let flag = TakerTraitsLib::post_interaction_call_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn need_check_epoch_manager(mut self) -> Self {
        let flag = TakerTraitsLib::need_check_epoch_manager_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn with_extension(mut self) -> Self {
        let flag = TakerTraitsLib::has_extension_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn use_permit2(mut self) -> Self {
        let flag = TakerTraitsLib::use_permit2_maker_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn unwrap_weth(mut self) -> Self {
        let flag = TakerTraitsLib::unwrap_weth_maker_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn build(self) -> U256 {
        self.traits
    }
}
