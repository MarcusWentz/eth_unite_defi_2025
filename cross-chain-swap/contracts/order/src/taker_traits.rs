use soroban_sdk::{contract, contractimpl, Address, Env, U256};

use crate::consts_trait::{u256_bitwise_and, ConstTrait};

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
#[contract]
pub struct TakerTraitsLib;

impl ConstTrait for TakerTraitsLib {}

#[contractimpl]
impl TakerTraitsLib {
    /// Checks if the args should contain target address.
    fn args_has_target(env: Env, taker_traits: U256) -> bool {
        u256_bitwise_and(
            &env,
            &taker_traits,
            &Self::args_has_target_const(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Retrieves the length of the extension calldata from the takerTraits.
    fn args_extension_length(env: Env, taker_traits: U256) -> U256 {
        u256_bitwise_and(
            &env,
            &taker_traits.shr(Self::ARGS_EXTENSION_LENGTH_OFFSET),
            &U256::from_u32(&env, Self::ARGS_EXTENSION_LENGTH_MASK),
        )
    }

    /// Retrieves the length of the interaction calldata from the takerTraits.
    fn args_interaction_length(env: Env, taker_traits: U256) -> U256 {
        u256_bitwise_and(
            &env,
            &taker_traits.shr(Self::ARGS_INTERACTION_LENGTH_OFFSET),
            &U256::from_u32(&env, Self::ARGS_INTERACTION_LENGTH_MASK),
        )
    }

    /// Checks if the taking amount should be calculated based on making amount.
    fn is_making_amount(env: Env, taker_traits: U256) -> bool {
        u256_bitwise_and(&env, &taker_traits, &Self::maker_amount_flag(env.clone()))
            .ne(&U256::from_u32(&env, 0))
    }

    /// Checks if the order should unwrap WETH and send ETH to taker.
    fn unwrap_weth(env: Env, taker_traits: U256) -> bool {
        u256_bitwise_and(
            &env,
            &taker_traits,
            &Self::unwrap_weth_taker_flag(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Checks if the order should skip maker's permit execution.
    fn skip_maker_permit(env: Env, taker_traits: U256) -> bool {
        u256_bitwise_and(
            &env,
            &taker_traits,
            &Self::skip_order_permit_flag(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Checks if the order uses the permit2 instead of permit.
    fn use_permit2(env: Env, taker_traits: U256) -> bool {
        u256_bitwise_and(
            &env,
            &taker_traits,
            &Self::use_permit2_taker_flag(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Retrieves the threshold amount from the takerTraits.
    /// The maximum amount a taker agrees to give in exchange for a making amount.
    fn threshold(env: Env, taker_traits: U256) -> U256 {
        u256_bitwise_and(&env, &taker_traits, &Self::amount_mask(env.clone()))
    }
}
