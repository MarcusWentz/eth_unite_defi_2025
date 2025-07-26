#![no_std]
use super::merkle_proof::process_proof;
use soroban_sdk::{
    contract, contractimpl, contracttype, log, symbol_short, Bytes, BytesN, Env, Symbol, Vec, U256,
};

const LAST_VALIDATED: Symbol = symbol_short!("VALIDATED");

#[contract]
pub struct MerkleStorageInvalidatorContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationData {
    pub index: U256, // must be uint256
    pub leaf: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TakerData {
    pub proof: Vec<BytesN<32>>,
    pub idx: U256,
    pub secret_hash: BytesN<32>,
}

#[contractimpl]
impl MerkleStorageInvalidatorContract {
    /// Set validation data for a given key (equivalent to mapping assignment)
    pub fn set_last_validated(
        env: Env,
        key: Symbol,
        validation_data: ValidationData,
    ) -> ValidationData {
        env.storage().persistent().set(&key, &validation_data);

        env.storage().persistent().extend_ttl(&key, 100, 1000);
        validation_data
    }

    /// Get validation data for a given key (equivalent to mapping access)
    pub fn get_last_validated(env: Env, key: Symbol) -> Option<ValidationData> {
        env.storage().persistent().get(&key).unwrap_or(None)
    }

    pub fn taker_interaction(
        env: Env,
        order_hash: BytesN<32>,
        // already extracted from extraDataArgs
        root_shortened: BytesN<32>, // diferent from MerkleStorageInvalidator.sol
        taker_data: TakerData,
    ) -> BytesN<32> {
        let root_calculated = process_proof(&env, &taker_data.proof, taker_data.secret_hash);

        if root_calculated != root_shortened {
            panic!("Invalid proof");
        }

        // Create storage key by hashing order_hash and root_shortened
        let mut combined = Bytes::new(&env);
        combined.extend_from_array(&order_hash.to_array());
        combined.extend_from_array(&root_shortened.to_array());

        let key = env.crypto().keccak256(&combined).into();

        log!(&env, "key: {:?}", key);
        key
    }
}
