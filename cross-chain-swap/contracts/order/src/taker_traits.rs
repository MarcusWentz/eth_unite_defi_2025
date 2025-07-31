use soroban_sdk::{contract, contractimpl, xdr::ToXdr, Address, Env, U256};

use crate::consts_trait::ConstTrait;

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
    // Helper function to check if a specific bit flag is set
    fn check_flag(env: Env, taker_traits: U256, bit_position: U256) -> bool {
        // Create a mask with the bit set at the specified position
        let mask = U256::from_u32(&env, 1).shl(bit_position.to_u128().unwrap() as u32);
        u256_bitwise_and(&env, &taker_traits, &mask) != U256::from_u32(&env, 0)
    }

    /// Checks if the taker uses permit2.
    pub fn use_permit2(env: Env, taker_traits: U256) -> bool {
        Self::check_flag(env.clone(), taker_traits, Self::use_permit2_taker_flag(env))
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

/// More efficient bitwise AND implementation
fn u256_bitwise_and(env: &Env, a: &U256, b: &U256) -> U256 {
    let mut result = U256::from_u32(env, 0);
    let mut a_temp = a.clone();
    let mut b_temp = b.clone();
    let mut power = U256::from_u32(env, 1);

    // Process each bit
    for _ in 0..256 {
        // Get the least significant bit of each number
        let a_lsb = a_temp.rem_euclid(&U256::from_u32(env, 2));
        let b_lsb = b_temp.rem_euclid(&U256::from_u32(env, 2));

        // If both bits are 1, add the power to result
        if a_lsb == U256::from_u32(env, 1) && b_lsb == U256::from_u32(env, 1) {
            result = result.add(&power);
        }

        // Right shift both numbers by 1
        a_temp = a_temp.div(&U256::from_u32(env, 2));
        b_temp = b_temp.div(&U256::from_u32(env, 2));

        // Move to next bit position
        power = power.mul(&U256::from_u32(env, 2));
    }

    result
}
