use soroban_sdk::{contract, contractimpl, xdr::ToXdr, Address, Env, U256};

use crate::consts_trait::ConstTrait;
use utils::math::bitand;

/// MakerTraitsLib equivalent for Soroban
///
/// The MakerTraits type is a U256 and different parts of the number are used to encode different traits.
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
pub struct MakerTraitsLib;

impl ConstTrait for MakerTraitsLib {}

#[contractimpl]
impl MakerTraitsLib {
    /// Checks if the order has the extension flag set.
    /// If the `HAS_EXTENSION_FLAG` is set in the maker_traits, then the protocol expects that the order has extension(s).
    pub fn has_extension(env: Env, maker_traits: U256) -> bool {
        bitand(&env, maker_traits, Self::has_extension_flag(env.clone()))
            .ne(&U256::from_u32(&env, 0))
    }

    /// Checks if the maker allows a specific taker to fill the order.
    pub fn is_allowed_sender(env: &Env, maker_traits: U256, sender: Address) -> bool {
        let maker_traits = maker_traits;
        let allowed_sender_bits = Self::extract_low_bits(env, maker_traits, 0, 80);

        if allowed_sender_bits == 0 {
            return true; // Any sender allowed
        }

        // Convert address to bytes and take last 10 bytes (80 bits)
        let sender_bytes = sender.to_string().to_xdr(&env);
        let sender_len = sender_bytes.len();

        if sender_len < 10 {
            return false;
        }

        // Extract last 10 bytes and convert to u128
        let mut sender_bits: u128 = 0;
        for i in 0..10 {
            sender_bits =
                (sender_bits << 8) | (sender_bytes.get(sender_len - 10 + i).unwrap() as u128);
        }

        let sender_u256 = U256::from_u128(&env, sender_bits);
        let masked_sender = bitand(&env, sender_u256, Self::allowed_sender_mask(env.clone()));

        allowed_sender_bits as u128 == masked_sender.to_u128().unwrap()
    }

    /// Checks if the order has expired.
    pub fn is_expired(env: &Env, maker_traits: U256) -> bool {
        let expiration = Self::extract_low_bits(env, maker_traits, Self::EXPIRATION_OFFSET, 40);

        if expiration == 0 {
            return false; // No expiration set
        }

        let current_timestamp = env.ledger().timestamp();
        expiration < current_timestamp
    }

    /// Returns the nonce or epoch of the order.
    pub fn nonce_or_epoch(env: &Env, maker_traits: U256) -> u64 {
        Self::extract_low_bits(env, maker_traits, Self::NONCE_OR_EPOCH_OFFSET, 40)
    }

    /// Returns the series of the order.
    pub fn series(env: &Env, maker_traits: U256) -> u64 {
        Self::extract_low_bits(env, maker_traits, Self::SERIES_OFFSET, 40)
    }

    /// Determines if the order allows partial fills.
    /// If the NO_PARTIAL_FILLS_FLAG is not set in the maker_traits, then the order allows partial fills.
    pub fn allow_partial_fills(env: &Env, maker_traits: U256) -> bool {
        !bitand(&env, maker_traits, Self::no_partial_fills_flag(env.clone()))
            .ne(&U256::from_u32(&env, 0))
    }

    /// Checks if the maker needs pre-interaction call.
    pub fn need_pre_interaction_call(env: &Env, maker_traits: U256) -> bool {
        bitand(
            &env,
            maker_traits,
            Self::pre_interaction_call_flag(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Checks if the maker needs post-interaction call.
    pub fn need_post_interaction_call(env: &Env, maker_traits: U256) -> bool {
        bitand(
            &env,
            maker_traits,
            Self::post_interaction_call_flag(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Determines if the order allows multiple fills.
    /// If the ALLOW_MULTIPLE_FILLS_FLAG is set in the maker_traits, then the maker allows multiple fills.
    pub fn allow_multiple_fills(env: &Env, maker_traits: U256) -> bool {
        bitand(
            &env,
            maker_traits,
            Self::allow_multiple_fills_flag(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Determines if an order should use the bit invalidator or remaining amount validator.
    /// The bit invalidator can be used if the order does not allow partial or multiple fills.
    pub fn use_bit_invalidator(env: &Env, maker_traits: U256) -> bool {
        !Self::allow_partial_fills(env, maker_traits.clone())
            || !Self::allow_multiple_fills(env, maker_traits)
    }

    /// Checks if the maker needs to check the epoch.
    pub fn need_check_epoch_manager(env: &Env, maker_traits: U256) -> bool {
        bitand(
            &env,
            maker_traits,
            Self::need_check_epoch_manager_flag(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Checks if the maker uses permit2.
    pub fn use_permit2(env: Env, maker_traits: U256) -> bool {
        bitand(
            &env,
            maker_traits,
            Self::use_permit2_maker_flag(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    /// Checks if the maker needs to unwrap WETH.
    pub fn unwrap_weth(env: Env, maker_traits: U256) -> bool {
        bitand(
            &env,
            maker_traits,
            Self::unwrap_weth_maker_flag(env.clone()),
        )
        .ne(&U256::from_u32(&env, 0))
    }

    // Helper function to extract bits from the lower part of the U256
    pub fn extract_low_bits(env: &Env, maker_traits: U256, offset: u32, num_bits: u32) -> u64 {
        // Right shift to get the bits we want
        let shifted = maker_traits.shr(offset);

        // Create mask for the number of bits we want
        let mask = U256::from_u32(&env, 1)
            .shl(num_bits)
            .sub(&U256::from_u32(&env, 1));

        // Apply mask using our bitwise AND
        let result = bitand(&env, shifted, mask);

        // Convert to u64 (assuming it fits)
        result.to_u128().unwrap() as u64
    }
}

// Builder pattern for constructing MakerTraits
pub struct MakerTraitsBuilder {
    traits: U256,
    env: Env,
}

impl MakerTraitsBuilder {
    pub fn new(env: Env) -> Self {
        Self {
            traits: U256::from_u32(&env, 0),
            env,
        }
    }

    pub fn with_allowed_sender(mut self, sender_bits: u128) -> Self {
        let sender_u256 = U256::from_u128(&self.env, sender_bits);
        let mask = MakerTraitsLib::allowed_sender_mask(self.env.clone());
        let masked_sender = bitand(&self.env, sender_u256, mask);
        self.traits = self.traits.add(&masked_sender);
        self
    }

    pub fn with_expiration(mut self, expiration: u64) -> Self {
        let expiration_u256 = U256::from_u128(&self.env, expiration as u128);
        let mask = MakerTraitsLib::expiration_mask(self.env.clone());
        let masked_expiration = bitand(&self.env, expiration_u256, mask);
        let shifted = masked_expiration.shl(MakerTraitsLib::EXPIRATION_OFFSET);
        self.traits = self.traits.add(&shifted);
        self
    }

    pub fn with_nonce_or_epoch(mut self, nonce_or_epoch: u64) -> Self {
        let nonce_u256 = U256::from_u128(&self.env, nonce_or_epoch as u128);
        let mask = MakerTraitsLib::nonce_or_epoch_mask(self.env.clone());
        let masked_nonce = bitand(&self.env, nonce_u256, mask);
        let shifted = masked_nonce.shl(MakerTraitsLib::NONCE_OR_EPOCH_OFFSET);
        self.traits = self.traits.add(&shifted);
        self
    }

    pub fn with_series(mut self, series: u64) -> Self {
        let series_u256 = U256::from_u128(&self.env, series as u128);
        let mask = MakerTraitsLib::series_mask(self.env.clone());
        let masked_series = bitand(&self.env, series_u256, mask);
        let shifted = masked_series.shl(MakerTraitsLib::SERIES_OFFSET);
        self.traits = self.traits.add(&shifted);
        self
    }

    pub fn no_partial_fills(mut self) -> Self {
        let flag = MakerTraitsLib::no_partial_fills_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn allow_multiple_fills(mut self) -> Self {
        let flag = MakerTraitsLib::allow_multiple_fills_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn with_pre_interaction_call(mut self) -> Self {
        let flag = MakerTraitsLib::pre_interaction_call_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn with_post_interaction_call(mut self) -> Self {
        let flag = MakerTraitsLib::post_interaction_call_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn need_check_epoch_manager(mut self) -> Self {
        let flag = MakerTraitsLib::need_check_epoch_manager_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn with_extension(mut self) -> Self {
        let flag = MakerTraitsLib::has_extension_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn use_permit2(mut self) -> Self {
        let flag = MakerTraitsLib::use_permit2_maker_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn unwrap_weth(mut self) -> Self {
        let flag = MakerTraitsLib::unwrap_weth_maker_flag(self.env.clone());
        self.traits = self.traits.add(&flag);
        self
    }

    pub fn build(self) -> U256 {
        self.traits
    }
}
