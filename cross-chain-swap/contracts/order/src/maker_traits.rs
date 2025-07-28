use soroban_sdk::{Address, Env, U256};

pub type MakerTraits = U256;

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
pub struct MakerTraitsLib;

impl MakerTraitsLib {
    // Low 200 bits are used for allowed sender, expiration, nonce_or_epoch, and series
    const ALLOWED_SENDER_MASK: U256 = (U256::from_u32(1) << 80) - 1; // type(uint80).max
    const EXPIRATION_OFFSET: U256 = 80;
    const EXPIRATION_MASK: U256 = (U256::from_u32(1) << 40) - 1; // type(uint40).max
    const NONCE_OR_EPOCH_OFFSET: U256 = 120;
    const NONCE_OR_EPOCH_MASK: U256 = (U256::from_u32(1) << 40) - 1; // type(uint40).max
    const SERIES_OFFSET: U256 = 160;
    const SERIES_MASK: U256 = (U256::from_u32(1) << 40) - 1; // type(uint40).max

    // Flag constants
    const NO_PARTIAL_FILLS_FLAG: U256 = U256::from_u32(1) << 255;
    const ALLOW_MULTIPLE_FILLS_FLAG: U256 = U256::from_u32(1) << 254;
    const PRE_INTERACTION_CALL_FLAG: U256 = U256::from_u32(1) << 252;
    const POST_INTERACTION_CALL_FLAG: U256 = U256::from_u32(1) << 251;
    const NEED_CHECK_EPOCH_MANAGER_FLAG: U256 = U256::from_u32(1) << 250;
    const HAS_EXTENSION_FLAG: U256 = U256::from_u32(1) << 249;
    const USE_PERMIT2_FLAG: U256 = U256::from_u32(1) << 248;
    const UNWRAP_WETH_FLAG: U256 = U256::from_u32(1) << 247;

    /// Checks if the order has the extension flag set.
    /// If the `HAS_EXTENSION_FLAG` is set in the maker_traits, then the protocol expects that the order has extension(s).
    pub fn has_extension(maker_traits: &MakerTraits) -> bool {
        Self::check_flag(maker_traits, Self::HAS_EXTENSION_FLAG)
    }

    /// Checks if the maker allows a specific taker to fill the order.
    pub fn is_allowed_sender(env: &Env, maker_traits: &MakerTraits, sender: &Address) -> bool {
        let allowed_sender_bits = Self::extract_low_bits(maker_traits, 0, 80);
        
        if allowed_sender_bits == 0 {
            return true; // Any sender allowed
        }

        // Convert address to bytes and take last 10 bytes (80 bits)
        let sender_bytes = sender.to_string().as_bytes(env);
        let sender_len = sender_bytes.len();
        
        if sender_len < 10 {
            return false;
        }

        // Extract last 10 bytes and convert to u128
        let mut sender_bits: u128 = 0;
        for i in 0..10 {
            sender_bits = (sender_bits << 8) | (sender_bytes.get(sender_len - 10 + i).unwrap() as u128);
        }

        allowed_sender_bits == (sender_bits & Self::ALLOWED_SENDER_MASK)
    }

    /// Checks if the order has expired.
    pub fn is_expired(env: &Env, maker_traits: &MakerTraits) -> bool {
        let expiration = Self::extract_low_bits(maker_traits, Self::EXPIRATION_OFFSET, 40);
        
        if expiration == 0 {
            return false; // No expiration set
        }

        let current_timestamp = env.ledger().timestamp();
        expiration < current_timestamp
    }

    /// Returns the nonce or epoch of the order.
    pub fn nonce_or_epoch(maker_traits: &MakerTraits) -> u64 {
        Self::extract_low_bits(maker_traits, Self::NONCE_OR_EPOCH_OFFSET, 40)
    }

    /// Returns the series of the order.
    pub fn series(maker_traits: &MakerTraits) -> u64 {
        Self::extract_low_bits(maker_traits, Self::SERIES_OFFSET, 40)
    }

    /// Determines if the order allows partial fills.
    /// If the NO_PARTIAL_FILLS_FLAG is not set in the maker_traits, then the order allows partial fills.
    pub fn allow_partial_fills(maker_traits: &MakerTraits) -> bool {
        !Self::check_flag(maker_traits, Self::NO_PARTIAL_FILLS_FLAG)
    }

    /// Checks if the maker needs pre-interaction call.
    pub fn need_pre_interaction_call(maker_traits: &MakerTraits) -> bool {
        Self::check_flag(maker_traits, Self::PRE_INTERACTION_CALL_FLAG)
    }

    /// Checks if the maker needs post-interaction call.
    pub fn need_post_interaction_call(maker_traits: &MakerTraits) -> bool {
        Self::check_flag(maker_traits, Self::POST_INTERACTION_CALL_FLAG)
    }

    /// Determines if the order allows multiple fills.
    /// If the ALLOW_MULTIPLE_FILLS_FLAG is set in the maker_traits, then the maker allows multiple fills.
    pub fn allow_multiple_fills(maker_traits: &MakerTraits) -> bool {
        Self::check_flag(maker_traits, Self::ALLOW_MULTIPLE_FILLS_FLAG)
    }

    /// Determines if an order should use the bit invalidator or remaining amount validator.
    /// The bit invalidator can be used if the order does not allow partial or multiple fills.
    pub fn use_bit_invalidator(maker_traits: &MakerTraits) -> bool {
        !Self::allow_partial_fills(maker_traits) || !Self::allow_multiple_fills(maker_traits)
    }

    /// Checks if the maker needs to check the epoch.
    pub fn need_check_epoch_manager(maker_traits: &MakerTraits) -> bool {
        Self::check_flag(maker_traits, Self::NEED_CHECK_EPOCH_MANAGER_FLAG)
    }

    /// Checks if the maker uses permit2.
    pub fn use_permit2(maker_traits: &MakerTraits) -> bool {
        Self::check_flag(maker_traits, Self::USE_PERMIT2_FLAG)
    }

    /// Checks if the maker needs to unwrap WETH.
    pub fn unwrap_weth(maker_traits: &MakerTraits) -> bool {
        Self::check_flag(maker_traits, Self::UNWRAP_WETH_FLAG)
    }

    // Helper function to check if a specific bit flag is set
    fn check_flag(maker_traits: &MakerTraits, bit_position: u32) -> bool {
        // Create a mask with the bit set at the specified position
        let mask = U256::from_u32(1) << bit_position;
        (*maker_traits & mask) != U256::from_u32(0)
    }

    // Helper function to extract bits from the lower part of the U256
    fn extract_low_bits(maker_traits: &MakerTraits, offset: u32, num_bits: u32) -> u64 {
        let mask = (1u64 << num_bits) - 1;
        let shifted = *maker_traits >> offset;
        
        // Convert to u64 for the low bits operations
        // This assumes the extracted value fits in u64, which is true for our use cases
        let as_u64 = shifted.lo().lo(); // Get the lowest 64 bits
        as_u64 & mask
    }
}

// Builder pattern for constructing MakerTraits
pub struct MakerTraitsBuilder {
    traits: U256,
}

impl MakerTraitsBuilder {
    pub fn new() -> Self {
        Self {
            traits: U256::from_u32(0),
        }
    }

    pub fn with_allowed_sender(mut self, sender_bits: u128) -> Self {
        let masked_sender = sender_bits & MakerTraitsLib::ALLOWED_SENDER_MASK;
        self.traits = self.traits | U256::from_u128(masked_sender);
        self
    }

    pub fn with_expiration(mut self, expiration: u64) -> Self {
        let masked_expiration = expiration & MakerTraitsLib::EXPIRATION_MASK;
        let shifted = U256::from_u64(masked_expiration) << MakerTraitsLib::EXPIRATION_OFFSET;
        self.traits = self.traits | shifted;
        self
    }

    pub fn with_nonce_or_epoch(mut self, nonce_or_epoch: u64) -> Self {
        let masked_nonce = nonce_or_epoch & MakerTraitsLib::NONCE_OR_EPOCH_MASK;
        let shifted = U256::from_u64(masked_nonce) << MakerTraitsLib::NONCE_OR_EPOCH_OFFSET;
        self.traits = self.traits | shifted;
        self
    }

    pub fn with_series(mut self, series: u64) -> Self {
        let masked_series = series & MakerTraitsLib::SERIES_MASK;
        let shifted = U256::from_u64(masked_series) << MakerTraitsLib::SERIES_OFFSET;
        self.traits = self.traits | shifted;
        self
    }

    pub fn no_partial_fills(mut self) -> Self {
        self.traits = self.traits | (U256::from_u32(1) << MakerTraitsLib::NO_PARTIAL_FILLS_FLAG);
        self
    }

    pub fn allow_multiple_fills(mut self) -> Self {
        self.traits = self.traits | (U256::from_u32(1) << MakerTraitsLib::ALLOW_MULTIPLE_FILLS_FLAG);
        self
    }

    pub fn with_pre_interaction_call(mut self) -> Self {
        self.traits = self.traits | (U256::from_u32(1) << MakerTraitsLib::PRE_INTERACTION_CALL_FLAG);
        self
    }

    pub fn with_post_interaction_call(mut self) -> Self {
        self.traits = self.traits | (U256::from_u32(1) << MakerTraitsLib::POST_INTERACTION_CALL_FLAG);
        self
    }

    pub fn need_check_epoch_manager(mut self) -> Self {
        self.traits = self.traits | (U256::from_u32(1) << MakerTraitsLib::NEED_CHECK_EPOCH_MANAGER_FLAG);
        self
    }

    pub fn with_extension(mut self) -> Self {
        self.traits = self.traits | (U256::from_u32(1) << MakerTraitsLib::HAS_EXTENSION_FLAG);
        self
    }

    pub fn use_permit2(mut self) -> Self {
        self.traits = self.traits | (U256::from_u32(1) << MakerTraitsLib::USE_PERMIT2_FLAG);
        self
    }

    pub fn unwrap_weth(mut self) -> Self {
        self.traits = self.traits | (U256::from_u32(1) << MakerTraitsLib::UNWRAP_WETH_FLAG);
        self
    }

    pub fn build(self) -> MakerTraits {
        self.traits
    }
}



