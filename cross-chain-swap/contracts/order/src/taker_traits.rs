use soroban_sdk::{contract, contractimpl, xdr::ToXdr, Address, Env, U256};

use crate::{
    consts_trait::{u256_bitwise_and, ConstTrait},
    taker_traits,
};

/// TakerTraitsLib equivalent for Soroban
///
/// The TakerTraits type is a U256 and different parts of the number are used to encode different traits.
/// High bits are used for flags:
/// 255 bit `NO_PARTIAL_FILLS_FLAG`          - if set, the order does not allow partial fills
/// 254 bit `ALLOW_MULTIPLE_FILLS_FLAG`      - if set, the order permits multiple fills
/// 253 bit                                  - unused
/// 252 bit `PRE_INTERACTION_CALL_FLAG`      - if set, the order requires pre-interaction call
/// 251 bit `POST_INTERACTION_CALL_FLAG`     - if set, the order requires post-interaction call
/// 250 bit `NEED_CHECK_EPOCH_MANAGER_FLAG`  - if set, the order requires to check the epoch manager
/// 249 bit `HAS_EXTENSION_FLAG`             - if set, the order has extension(s)
/// 248 bit `USE_PERMIT2_FLAG`               - if set, the order uses permit2
/// 247 bit `UNWRAP_WETH_FLAG`               - if set, the order requires to unwrap WETH
///
/// Low 200 bits are used for allowed sender, expiration, nonce_or_epoch, and series:
/// uint80 last 10 bytes of allowed sender address (0 if any)
/// uint40 expiration timestamp (0 if none)
/// uint40 nonce or epoch
/// uint40 series
#[contract]
pub struct TakerTraitsLib;

impl ConstTrait for TakerTraitsLib {}

#[contractimpl]
impl TakerTraitsLib {
    fn args_has_target(env: Env, taker_traits: U256) -> bool {
        u256_bitwise_and(
            &env,
            &taker_traits,
            &Self::args_has_target_const(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    fn args_extension_length(env: Env, taker_traits: U256) -> U256 {
        u256_bitwise_and(
            &env,
            &taker_traits.shr(Self::ARGS_EXTENSION_LENGTH_OFFSET),
            &U256::from_u32(&env, Self::ARGS_EXTENSION_LENGTH_MASK),
        )
    }

    fn args_interaction_length(env: Env, taker_traits: U256) -> U256 {
        u256_bitwise_and(
            &env,
            &taker_traits.shr(Self::ARGS_INTERACTION_LENGTH_OFFSET),
            &U256::from_u32(&env, Self::ARGS_INTERACTION_LENGTH_MASK),
        )
    }

    /// Checks if the taker uses permit2.
    fn is_making_amount(env: Env, taker_traits: U256) -> bool {
        Self::check_flag(env.clone(), taker_traits, Self::maker_amount_flag(env))
    }

    /// Checks if the taker uses permit2.
    fn skip_maker_permit(env: Env, taker_traits: U256) -> bool {
        Self::check_flag(env.clone(), taker_traits, Self::skip_order_permit_flag(env))
    }

    /// Checks if the taker uses permit2.
    fn use_permit2(env: Env, taker_traits: U256) -> bool {
        Self::check_flag(env.clone(), taker_traits, Self::use_permit2_taker_flag(env))
    }

    /// Checks if the taker needs to unwrap WETH.
    fn unwrap_weth(env: Env, taker_traits: U256) -> bool {
        Self::check_flag(env.clone(), taker_traits, Self::unwrap_weth_taker_flag(env))
    }

    fn treshold(env: Env, taker_traits: U256) -> U256 {
        u256_bitwise_and(&env, &taker_traits, &Self::amount_mask(env.clone()))
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

    pub fn use_permit2(mut self) -> Self {
        let flag = TakerTraitsLib::use_permit2_taker_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn unwrap_weth(mut self) -> Self {
        let flag = TakerTraitsLib::unwrap_weth_taker_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn build(self) -> U256 {
        self.traits
    }
}
