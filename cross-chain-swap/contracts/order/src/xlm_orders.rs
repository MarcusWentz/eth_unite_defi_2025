use crate::maker_traits::MakerTraitsLib;
use base_escrow::base_escrow::BaseEscrow;
use order_interface::Order;
use soroban_sdk::{
    contract, contracttype, symbol_short,
    xdr::{FromXdr, ToXdr},
    Address, Bytes, BytesN, Env, Symbol, Vec, U256,
};
use utils::math::bitand;

#[contract]
pub struct XLMOrders;

impl BaseEscrow for XLMOrders {}

#[derive(Eq, PartialEq, Debug, Clone)]
#[contracttype]
pub struct XLMOrdersArr {
    pub maker: Address,
    pub balance: u128,
    pub maximum_premium: u32,
    pub auction_duration: u32,
}

#[derive(Eq, PartialEq, Debug)]
#[contracttype]
pub enum ValidationResult {
    MissingOrderExtension,
    InvalidExtensionHash,
    UnexpectedOrderExtension,
    Success,
}

#[contracttype]
pub enum DynamicField {
    MakerAssetSuffix,
    TakerAssetSuffix,
    MakingAmountData,
    TakingAmountData,
    Predicate,
    MakerPermit,
    PreInteractionData,
    PostInteractionData,
    CustomData,
}

// EVENTS SYMBOLS
const XLM_DEPOSITED: Symbol = symbol_short!("XLM_DEP");
const XLM_ORDER_CANCELLED: Symbol = symbol_short!("XLM_OC");
const _XLM_ORDER_CANCELLED_BY_THIRD_PARTY: Symbol = symbol_short!("XLM_OC3");

// STORAGE SYMBOLS
const LIMIT_ORDER_PROTOCOL: Symbol = symbol_short!("LIM_ORP");
const XLM: Symbol = symbol_short!("XLM");
const ACCESS_TOKEN: Symbol = symbol_short!("ACC_TOK");

// Consts
const _PREMIUM_BASE: u32 = 1_000;
const _CANCEL_GAS_LOWER_BOUND: u32 = 30_000;

// OrderLib constants
const LIMIT_ORDER_TYPEHASH: &str = "Order(uint256 salt,address maker,address receiver,address makerAsset,address takerAsset,uint256 makingAmount,uint256 takingAmount,uint256 makerTraits)";
const _ORDER_STRUCT_SIZE: u32 = 0x100;
const DATA_HASH_SIZE: u32 = 0x120;

// EIP-712 constants
const EIP712_DOMAIN_TYPEHASH: &str =
    "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";

impl XLMOrders {
    pub fn constructor(
        env: Env,
        xml: Address,
        limit_order_protocol: Address,
        access_token: Address,
    ) {
        env.storage().instance().set(&XLM, &xml);
        env.storage()
            .instance()
            .set(&LIMIT_ORDER_PROTOCOL, &limit_order_protocol);
        env.storage().instance().set(&ACCESS_TOKEN, &access_token);
    }

    pub fn xlm_orders_batch(env: Env, order_hashes: Vec<BytesN<32>>) -> Vec<XLMOrdersArr> {
        let mut res: Vec<XLMOrdersArr> = Vec::new(&env);
        for i in 0..order_hashes.len() {
            let order_hash = order_hashes.get(i).unwrap();
            let order_data: Option<XLMOrdersArr> = env.storage().instance().get(&order_hash);
            if let Some(data) = order_data {
                res.push_back(data);
            }
        }
        res
    }

    pub fn xlm_order_deposit(
        env: Env,
        order: Order,
        extension: Bytes,
        maximum_premium: u32,
        auction_duration: u32,
    ) -> BytesN<32> {
        if MakerTraitsLib::need_post_interaction_call(&env, order.maker_traits.clone()) {
            panic!("InvalidOrder")
        }

        let (valid, validation_result) =
            is_valid_extension(env.clone(), order.clone(), extension.clone());
        if !valid {
            match validation_result {
                ValidationResult::MissingOrderExtension => panic!("MissingOrderExtension"),
                ValidationResult::InvalidExtensionHash => panic!("InvalidExtensionHash"),
                ValidationResult::UnexpectedOrderExtension => panic!("UnexpectedOrderExtension"),
                ValidationResult::Success => (),
            }
        }

        if order.maker != env.current_contract_address() {
            panic!("AccessDenied")
        }

        if order.receiver
            != env
                .storage()
                .instance()
                .get(&symbol_short!("sender"))
                .unwrap()
        {
            panic!("AccessDenied")
        }

        if order.making_amount
            != env
                .storage()
                .instance()
                .get(&symbol_short!("value"))
                .unwrap()
        {
            panic!("InvalidOrder")
        }

        let interaction = post_interaction_target_and_data(&env, &order, &extension);

        if interaction.len() != 20
            || Address::from_xdr(&env, &interaction).unwrap()
                != env
                    .storage()
                    .instance()
                    .get(&symbol_short!("sender"))
                    .unwrap()
        {
            panic!("InvalidOrder")
        }

        let order_hash = hash(&env, &order, &domain_separator_v4(&env));

        if env
            .storage()
            .instance()
            .get::<_, XLMOrdersArr>(&order_hash)
            .unwrap()
            != env
                .storage()
                .instance()
                .get(&symbol_short!("sender"))
                .unwrap()
        {
            panic!("ExistingOrder")
        }

        let order_data = XLMOrdersArr {
            maker: env
                .storage()
                .instance()
                .get(&symbol_short!("sender"))
                .unwrap(),
            balance: env
                .storage()
                .instance()
                .get(&symbol_short!("value"))
                .unwrap(),
            maximum_premium: maximum_premium,
            auction_duration: auction_duration,
        };

        env.storage().instance().set(&order_hash, &order_data);

        Self::uni_transfer(
            env.clone(),
            env.storage().instance().get(&XLM).unwrap(),
            env.storage()
                .instance()
                .get(&symbol_short!("sender"))
                .unwrap(),
            env.storage()
                .instance()
                .get(&symbol_short!("value"))
                .unwrap(),
        );

        env.events().publish(
            (XLM_DEPOSITED, symbol_short!("Deposited")),
            env.storage()
                .instance()
                .get::<_, Address>(&symbol_short!("value")),
        );
        order_hash
    }

    pub fn cancel_order(env: Env, _maker_trairs: U256, order_hash: BytesN<32>) {
        if env
            .storage()
            .instance()
            .get::<_, XLMOrdersArr>(&order_hash)
            .unwrap()
            .maker
            != env
                .storage()
                .instance()
                .get(&symbol_short!("sender"))
                .unwrap()
        {
            panic!("InvalidOrder")
        }
        let refund_xlm_amount = env
            .storage()
            .instance()
            .get::<_, XLMOrdersArr>(&order_hash)
            .unwrap()
            .balance as i128;

        Self::uni_transfer(
            env.clone(),
            env.storage().instance().get(&XLM).unwrap(),
            env.storage()
                .instance()
                .get(&symbol_short!("sender"))
                .unwrap(),
            refund_xlm_amount,
        );

        env.events().publish(
            (XLM_ORDER_CANCELLED, symbol_short!("canceled")),
            (order_hash, refund_xlm_amount),
        );
    }

    /// Port of Solidity OrderMixin.cancelOrder() function
    /// Handles both bit invalidator and remaining invalidator cases
    pub fn cancel_order_mixin(env: Env, maker_traits: U256, order_hash: BytesN<32>) {
        let sender = env.current_contract_address();
        let order_hash_clone = order_hash.clone();

        // Check if order uses bit invalidator
        if MakerTraitsLib::use_bit_invalidator(&env, maker_traits.clone()) {
            // Handle bit invalidator case
            let nonce_or_epoch = MakerTraitsLib::nonce_or_epoch(&env, maker_traits.clone());

            // Mass invalidate orders for this nonce/epoch
            let invalidator_result =
                Self::mass_invalidate_bit_orders(&env, sender.clone(), nonce_or_epoch, 0);

            // Emit bit invalidator updated event
            env.events().publish(
                (symbol_short!("BIT_INV"), symbol_short!("updated")),
                (
                    sender.clone(),
                    U256::from_u128(&env, nonce_or_epoch as u128).shr(8),
                    invalidator_result,
                ),
            );
        } else {
            // Handle remaining invalidator case (fully fill the order)
            Self::fully_fill_remaining_order(&env, sender.clone(), order_hash_clone.clone());

            // Emit order cancelled event
            env.events().publish(
                (XLM_ORDER_CANCELLED, symbol_short!("cancelled")),
                order_hash_clone,
            );
        }
    }

    pub fn _get_current_premium_multiplier(
        env: Env,
        order: XLMOrdersArr,
        expiration_time: U256,
    ) -> U256 {
        let timestamp = U256::from_u128(&env, env.ledger().timestamp() as u128);
        if timestamp.le(&expiration_time) {
            return U256::from_u32(&env, 0);
        }

        let time_elapsed = timestamp.sub(&expiration_time);

        if time_elapsed.ge(&U256::from_u32(&env, order.auction_duration)) {
            return U256::from_u32(&env, order.maximum_premium);
        }

        return time_elapsed
            .mul(&U256::from_u32(&env, order.maximum_premium))
            .div(&U256::from_u32(&env, order.auction_duration));
    }

    /// Mass invalidate bit orders for a given nonce/epoch
    /// This is equivalent to the Solidity bit invalidator mass invalidation
    /// Based on BitInvalidatorLib.massInvalidate logic
    pub fn mass_invalidate_bit_orders(
        env: &Env,
        _maker: Address,
        nonce_or_epoch: u64,
        _series: u64,
    ) -> U256 {
        let nonce_u256 = U256::from_u128(env, nonce_or_epoch as u128);
        let _invalidator_slot = nonce_u256.shr(8);
        let invalidator_bits = U256::from_u32(env, 1).shl(
            bitand(env, nonce_u256, U256::from_u32(env, 0xff))
                .to_u128()
                .unwrap() as u32,
        );
        let result = env
            .storage()
            .instance()
            .get::<_, U256>(&symbol_short!("BIT_INV"))
            .unwrap_or(U256::from_u32(env, 0))
            .add(&invalidator_bits);
        env.storage()
            .instance()
            .set(&symbol_short!("BIT_INV"), &result);
        result
    }

    /// Fully fill a remaining order (mark it as completely filled)
    /// This is equivalent to RemainingInvalidatorLib.fullyFilled()
    pub fn fully_fill_remaining_order(env: &Env, _maker: Address, order_hash: BytesN<32>) {
        // Set the remaining invalidator to type(uint256).max (fully filled)
        // This is equivalent to RemainingInvalidatorLib.fullyFilled()
        let fully_filled_invalidator = U256::from_u128(env, u128::MAX);
        env.storage()
            .instance()
            .set(&order_hash, &fully_filled_invalidator);
    }
}

pub fn is_valid_extension(env: Env, order: Order, extension: Bytes) -> (bool, ValidationResult) {
    if MakerTraitsLib::has_extension(env.clone(), order.maker_traits) {
        if extension.len() == 0 {
            return (false, ValidationResult::MissingOrderExtension);
        }
        if bitand(
            &env,
            U256::from_be_bytes(&env, &env.crypto().keccak256(&extension).to_xdr(&env)),
            U256::from_u128(&env, u128::MAX),
        ) != bitand(&env, order.salt, U256::from_u128(&env, u128::MAX))
        {
            return (false, ValidationResult::InvalidExtensionHash);
        }
    } else {
        if extension.len() > 0 {
            return (false, ValidationResult::UnexpectedOrderExtension);
        }
    }
    (true, ValidationResult::Success)
}

pub fn post_interaction_target_and_data(env: &Env, _order: &Order, extension: &Bytes) -> Bytes {
    get_extension(env, extension, DynamicField::PostInteractionData)
}

pub fn get_extension(env: &Env, extension: &Bytes, field: DynamicField) -> Bytes {
    // Check if extension has at least 32 bytes (0x20)
    if extension.len() < 32 {
        return Bytes::new(env);
    }

    // Extract offsets from the first 32 bytes
    let mut offsets_array = [0u8; 32];
    for i in 0..32 {
        offsets_array[i] = extension.get(i as u32).unwrap_or(0);
    }
    let offsets_bytes = Bytes::from_array(env, &offsets_array);
    let offsets = U256::from_be_bytes(env, &offsets_bytes);

    // Get the concatenated data (everything after the first 32 bytes)
    let concat_len = extension.len() - 32;
    let mut concat_data = Bytes::new(env);
    for i in 0..concat_len {
        concat_data.push_back(extension.get((i + 32) as u32).unwrap_or(0));
    }

    // Get the field data using the offsets
    get_field_from_offsets(env, offsets, concat_data, field)
}

fn get_field_from_offsets(env: &Env, offsets: U256, concat: Bytes, field: DynamicField) -> Bytes {
    let field_index = match field {
        DynamicField::MakerAssetSuffix => 0,
        DynamicField::TakerAssetSuffix => 1,
        DynamicField::MakingAmountData => 2,
        DynamicField::TakingAmountData => 3,
        DynamicField::Predicate => 4,
        DynamicField::MakerPermit => 5,
        DynamicField::PreInteractionData => 6,
        DynamicField::PostInteractionData => 7,
        DynamicField::CustomData => 8,
    };

    // Calculate bit shift: field_index * 32 (equivalent to shl(5, index))
    let bit_shift = field_index * 32;

    // Extract begin offset: and(0xffffffff, shr(bitShift, shl(32, offsets)))
    // This gets the 32 bits starting at bitShift position
    let begin = bitand(
        env,
        offsets.shl(32).shr(bit_shift),
        U256::from_u32(env, 0xffffffff),
    );

    // Extract end offset: and(0xffffffff, shr(bitShift, offsets))
    // This gets the 32 bits starting at bitShift position
    let end_offset = bitand(env, offsets.shr(bit_shift), U256::from_u32(env, 0xffffffff));

    // Convert to u32 for array indexing
    let begin_u32: u32 = begin.to_u128().unwrap().try_into().unwrap();
    let end_u32: u32 = end_offset.to_u128().unwrap().try_into().unwrap();

    // Validate bounds - equivalent to if gt(end, concat.length)
    if end_u32 > concat.len() as u32 {
        panic!("OffsetOutOfBounds");
    }

    // Extract the field data
    if begin_u32 >= end_u32 {
        return Bytes::new(env);
    }

    let start: usize = begin_u32 as usize;
    let end_pos: usize = end_u32 as usize;

    if start >= concat.len() as usize || end_pos > concat.len() as usize {
        panic!("OffsetOutOfBounds");
    }

    // Extract the field data using individual byte access
    let field_len = end_pos - start;
    let mut field_data = Bytes::new(env);
    for i in 0..field_len {
        field_data.push_back(concat.get((start + i) as u32).unwrap_or(0));
    }

    field_data
}

pub fn hash(env: &Env, order: &Order, domain_separator: &BytesN<32>) -> BytesN<32> {
    // First, calculate the keccak256 of the typehash
    let typehash_bytes = LIMIT_ORDER_TYPEHASH.as_bytes();
    let mut typehash_array = [0u8; 128]; // Fixed size array
    for i in 0..typehash_bytes.len().min(128) {
        typehash_array[i] = typehash_bytes[i];
    }
    let typehash = env
        .crypto()
        .keccak256(&Bytes::from_array(env, &typehash_array));

    // Create the order data for hashing
    let mut order_data = Bytes::from_array(env, &[0u8; DATA_HASH_SIZE as usize]);

    // Copy the typehash to the beginning
    let typehash_bytes = typehash.to_xdr(env);
    for i in 0..32 {
        order_data.set(i, typehash_bytes.get(i).unwrap_or(0));
    }

    // Copy the order fields (salt, maker, receiver, makerAsset, takerAsset, makingAmount, takingAmount, makerTraits)
    let salt_bytes = order.salt.to_be_bytes();
    for i in 0..32 {
        order_data.set(i + 32, salt_bytes.get(i).unwrap_or(0));
    }

    // Copy maker address (20 bytes)
    let maker_bytes = order.maker.clone().to_xdr(env);
    for i in 0..20 {
        order_data.set(i + 64, maker_bytes.get(i).unwrap_or(0));
    }

    // Copy receiver address (20 bytes)
    let receiver_bytes = order.receiver.clone().to_xdr(env);
    for i in 0..20 {
        order_data.set(i + 84, receiver_bytes.get(i).unwrap_or(0));
    }

    // Copy makerAsset address (20 bytes)
    let maker_asset_bytes = order.maker_asset.clone().to_xdr(env);
    for i in 0..20 {
        order_data.set(i + 104, maker_asset_bytes.get(i).unwrap_or(0));
    }

    // Copy takerAsset address (20 bytes)
    let taker_asset_bytes = order.taker_asset.clone().to_xdr(env);
    for i in 0..20 {
        order_data.set(i + 124, taker_asset_bytes.get(i).unwrap_or(0));
    }

    // Copy makingAmount (32 bytes)
    let making_amount_bytes = order.making_amount.to_be_bytes();
    for i in 0..32 {
        order_data.set(i + 144, making_amount_bytes.get(i).unwrap_or(0));
    }

    // Copy takingAmount (32 bytes)
    let taking_amount_bytes = order.taking_amount.to_be_bytes();
    for i in 0..32 {
        order_data.set(i + 176, taking_amount_bytes.get(i).unwrap_or(0));
    }

    // Copy makerTraits (32 bytes)
    let maker_traits_bytes = order.maker_traits.to_be_bytes();
    for i in 0..32 {
        order_data.set(i + 208, maker_traits_bytes.get(i).unwrap_or(0));
    }

    // Calculate the hash of the order data
    let order_hash = env.crypto().keccak256(&order_data);

    // Apply EIP-712 domain separator (equivalent to ECDSA.toTypedDataHash)
    let mut final_data = Bytes::from_array(env, &[0u8; 66]);

    // Add domain separator
    for i in 0..32 {
        final_data.set(i, domain_separator.get(i).unwrap_or(0));
    }

    // Add order hash
    let order_hash_bytes = order_hash.to_xdr(env);
    for i in 0..32 {
        final_data.set(i + 32, order_hash_bytes.get(i).unwrap_or(0));
    }

    // Add EIP-712 prefix
    final_data.set(32, 0x19); // EIP-712 prefix
    final_data.set(33, 0x01); // Version 1

    // Calculate final hash
    env.crypto().keccak256(&final_data).into()
}

pub fn domain_separator_v4(env: &Env) -> BytesN<32> {
    // Calculate hashed name and version
    let name = "XLMOrders";
    let version = "1.0.0";

    let name_bytes = name.as_bytes();
    let mut name_array = [0u8; 128];
    for i in 0..name_bytes.len() {
        name_array[i] = name_bytes[i];
    }
    let hashed_name = env
        .crypto()
        .keccak256(&Bytes::from_array(env, &name_array))
        .into();

    let version_bytes = version.as_bytes();
    let mut version_array = [0u8; 128];
    for i in 0..version_bytes.len() {
        version_array[i] = version_bytes[i];
    }
    let hashed_version = env
        .crypto()
        .keccak256(&Bytes::from_array(env, &version_array))
        .into();

    // Calculate type hash
    let type_hash_bytes = EIP712_DOMAIN_TYPEHASH.as_bytes();
    let mut type_hash_array = [0u8; 128];
    for i in 0..type_hash_bytes.len() {
        type_hash_array[i] = type_hash_bytes[i];
    }
    let type_hash = env
        .crypto()
        .keccak256(&Bytes::from_array(env, &type_hash_array))
        .into();

    // Build domain separator
    build_domain_separator(env, &type_hash, &hashed_name, &hashed_version)
}

fn build_domain_separator(
    env: &Env,
    type_hash: &BytesN<32>,
    name: &BytesN<32>,
    version: &BytesN<32>,
) -> BytesN<32> {
    // Create the domain data for hashing
    let mut domain_data = Bytes::from_array(env, &[0u8; 160]); // 5 * 32 bytes

    // Copy type hash (32 bytes)
    for i in 0..32 {
        domain_data.set(i, type_hash.get(i).unwrap_or(0));
    }

    // Copy hashed name (32 bytes)
    for i in 0..32 {
        domain_data.set(i + 32, name.get(i).unwrap_or(0));
    }

    // Copy hashed version (32 bytes)
    for i in 0..32 {
        domain_data.set(i + 64, version.get(i).unwrap_or(0));
    }

    // Copy chain ID (32 bytes) - for Soroban, we'll use a fixed value or get from env
    let chain_id = U256::from_u32(env, 1); // Default to 1, adjust as needed
    let chain_id_bytes = chain_id.to_be_bytes();
    for i in 0..32 {
        domain_data.set(i + 96, chain_id_bytes.get(i).unwrap_or(0));
    }

    // Copy verifying contract address (32 bytes)
    let contract_address = env.current_contract_address();
    let address_bytes = contract_address.to_xdr(env);
    for i in 0..32 {
        domain_data.set(i + 128, address_bytes.get(i).unwrap_or(0));
    }

    // Calculate the domain separator
    env.crypto().keccak256(&domain_data).into()
}

/**
 * Given an already hashed struct, this function returns the hash of the fully encoded EIP712 message for this domain.
 * This is the Soroban equivalent of the Solidity EIP712._hashTypedDataV4() function.
 *
 * @param env The Soroban environment
 * @param struct_hash The hashed struct
 * @return BytesN<32> The final EIP-712 hash
 */
pub fn hash_typed_data_v4(env: &Env, struct_hash: &BytesN<32>) -> BytesN<32> {
    let domain_separator = domain_separator_v4(env);

    // Create the final data for hashing (EIP-712 prefix + domain separator + struct hash)
    let mut final_data = Bytes::from_array(env, &[0u8; 66]); // 2 + 32 + 32 bytes

    // Add EIP-712 prefix
    final_data.set(0, 0x19); // EIP-712 prefix
    final_data.set(1, 0x01); // Version 1

    // Add domain separator
    for i in 0..32 {
        final_data.set(i + 2, domain_separator.get(i).unwrap_or(0));
    }

    // Add struct hash
    for i in 0..32 {
        final_data.set(i + 34, struct_hash.get(i).unwrap_or(0));
    }

    // Calculate final hash
    env.crypto().keccak256(&final_data).into()
}
