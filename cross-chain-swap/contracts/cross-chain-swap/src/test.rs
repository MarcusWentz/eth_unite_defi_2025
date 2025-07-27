#![cfg(test)]

// use super::*;
use crate::merkle_storage_invalidator::{commutative_keccak256, concat_bytes, process_proof, MerkleProof, MerkleStorageInvalidatorContract, MerkleStorageInvalidatorContractClient, ValidationData, TakerData, LAST_VALIDATED};
use soroban_sdk::{vec, BytesN, Env, U256, symbol_short};

#[test]
fn test_merkle_storage_invalidator() {
    let env = Env::default();
    let contract_id = env.register(MerkleStorageInvalidatorContract, ());
    let client = MerkleStorageInvalidatorContractClient::new(&env, &contract_id);

    assert_eq!(client.get_last_validated(&LAST_VALIDATED), None);

    let validation_data = ValidationData {
        index: U256::from_u32(&env, 1),
        leaf: BytesN::from_array(&env, &[0u8; 32]),
    };

    client.set_last_validated(&LAST_VALIDATED, &validation_data);

    assert_eq!(
        client.get_last_validated(&LAST_VALIDATED),
        Some(validation_data)
    );
}

#[test]
fn test_commutative_keccak256() {
    let env = Env::default();

    let a = BytesN::from_array(&env, &[1u8; 32]);
    let b = BytesN::from_array(&env, &[2u8; 32]);

    let result_ab = commutative_keccak256(&env, a.clone(), b.clone());
    let result_ba = commutative_keccak256(&env, b.clone(), a.clone());

    assert_eq!(result_ab, result_ba);
}

#[test]
fn test_commutative_keccak256_same_values() {
    let env = Env::default();

    let a = BytesN::from_array(&env, &[1u8; 32]);
    let b = BytesN::from_array(&env, &[1u8; 32]);

    let result = commutative_keccak256(&env, a.clone(), b.clone());
    let expected = concat_bytes(&env, a.clone(), b.clone());

    assert_eq!(result, expected);
}

#[test]
fn test_process_proof_empty() {
    let env = Env::default();

    let leaf = BytesN::from_array(&env, &[1u8; 32]);
    let empty_proof = vec![&env];

    let result = process_proof(&env, &empty_proof, leaf.clone());

    // With empty proof, result should be the leaf itself
    assert_eq!(result, leaf);
}

#[test]
fn test_process_proof_single_element() {
    let env = Env::default();

    let leaf = BytesN::from_array(&env, &[1u8; 32]);
    let proof_element = BytesN::from_array(&env, &[2u8; 32]);
    let proof = vec![&env, proof_element.clone()];

    let result = process_proof(&env, &proof, leaf.clone());
    let expected = commutative_keccak256(&env, leaf, proof_element);

    assert_eq!(result, expected);
}

#[test]
fn test_process_proof_multiple_elements() {
    let env = Env::default();

    let leaf = BytesN::from_array(&env, &[1u8; 32]);
    let proof1 = BytesN::from_array(&env, &[2u8; 32]);
    let proof2 = BytesN::from_array(&env, &[3u8; 32]);
    let proof = vec![&env, proof1.clone(), proof2.clone()];

    let result = process_proof(&env, &proof, leaf.clone());

    let step1 = commutative_keccak256(&env, leaf, proof1);
    let expected = commutative_keccak256(&env, step1, proof2);

    assert_eq!(result, expected);
}

#[test]
fn test_merkle_proof_verify_valid() {
    let env = Env::default();

    let leaf = BytesN::from_array(&env, &[1u8; 32]);
    let proof_element = BytesN::from_array(&env, &[2u8; 32]);
    let proof = vec![&env, proof_element.clone()];

    let root = commutative_keccak256(&env, leaf.clone(), proof_element);

    let is_valid = MerkleProof::verify(&env, &proof, root, leaf);

    assert!(is_valid);
}

#[test]
fn test_merkle_proof_verify_invalid() {
    let env = Env::default();

    let leaf = BytesN::from_array(&env, &[1u8; 32]);
    let proof_element = BytesN::from_array(&env, &[2u8; 32]);
    let proof = vec![&env, proof_element.clone()];

    let wrong_root = BytesN::from_array(&env, &[255u8; 32]);

    let is_valid = MerkleProof::verify(&env, &proof, wrong_root, leaf);

    assert!(!is_valid);
}

#[test]
fn test_taker_interaction_valid() {
    let env = Env::default();
    let contract_id = env.register(MerkleStorageInvalidatorContract, ());
    let client = MerkleStorageInvalidatorContractClient::new(&env, &contract_id);

    let order_hash = BytesN::from_array(&env, &[10u8; 32]);
    let secret_hash = BytesN::from_array(&env, &[1u8; 32]);
    let proof_element = BytesN::from_array(&env, &[2u8; 32]);
    let proof = vec![&env, proof_element.clone()];

    // Calculate the correct root
    let root = commutative_keccak256(&env, secret_hash.clone(), proof_element);

    let taker_data = TakerData {
        proof,
        idx: U256::from_u32(&env, 42),
        secret_hash: secret_hash.clone(),
    };

    let storage_key = client.taker_interaction(&order_hash, &root, &taker_data);

    // Verify the storage key was created
    assert!(storage_key.len() == 32);
}

#[test]
#[should_panic(expected = "Invalid proof")]
fn test_taker_interaction_invalid_proof() {
    let env = Env::default();
    let contract_id = env.register(MerkleStorageInvalidatorContract, ());
    let client = MerkleStorageInvalidatorContractClient::new(&env, &contract_id);

    let order_hash = BytesN::from_array(&env, &[10u8; 32]);
    let secret_hash = BytesN::from_array(&env, &[1u8; 32]);
    let proof_element = BytesN::from_array(&env, &[2u8; 32]);
    let proof = vec![&env, proof_element.clone()];

    // Use wrong root
    let wrong_root = BytesN::from_array(&env, &[255u8; 32]);

    let taker_data = TakerData {
        proof,
        idx: U256::from_u32(&env, 42),
        secret_hash,
    };

    // This should panic with "Invalid proof"
    client.taker_interaction(&order_hash, &wrong_root, &taker_data);
}

#[test]
fn test_validation_data_equality() {
    let env = Env::default();

    let data1 = ValidationData {
        index: U256::from_u32(&env, 1),
        leaf: BytesN::from_array(&env, &[1u8; 32]),
    };

    let data2 = ValidationData {
        index: U256::from_u32(&env, 1),
        leaf: BytesN::from_array(&env, &[1u8; 32]),
    };

    let data3 = ValidationData {
        index: U256::from_u32(&env, 2),
        leaf: BytesN::from_array(&env, &[1u8; 32]),
    };

    assert_eq!(data1, data2);
    assert_ne!(data1, data3);
}

#[test]
fn test_taker_data_creation() {
    let env = Env::default();

    let proof = vec![&env, BytesN::from_array(&env, &[1u8; 32])];
    let idx = U256::from_u32(&env, 100);
    let secret_hash = BytesN::from_array(&env, &[2u8; 32]);

    let taker_data = TakerData {
        proof: proof.clone(),
        idx: idx.clone(),
        secret_hash: secret_hash.clone(),
    };

    assert_eq!(taker_data.proof, proof);
    assert_eq!(taker_data.idx, idx);
    assert_eq!(taker_data.secret_hash, secret_hash);
}

#[test]
fn test_storage_persistence() {
    let env = Env::default();
    let contract_id = env.register(MerkleStorageInvalidatorContract, ());
    let client = MerkleStorageInvalidatorContractClient::new(&env, &contract_id);

    let key1 = symbol_short!("KEY1");
    let key2 = symbol_short!("KEY2");

    let data1 = ValidationData {
        index: U256::from_u32(&env, 1),
        leaf: BytesN::from_array(&env, &[1u8; 32]),
    };

    let data2 = ValidationData {
        index: U256::from_u32(&env, 2),
        leaf: BytesN::from_array(&env, &[2u8; 32]),
    };

    client.set_last_validated(&key1, &data1);
    client.set_last_validated(&key2, &data2);

    assert_eq!(client.get_last_validated(&key1), Some(data1));
    assert_eq!(client.get_last_validated(&key2), Some(data2));

    // Verify non-existent key returns None
    let key3 = symbol_short!("KEY3");
    assert_eq!(client.get_last_validated(&key3), None);
}

#[test]
fn test_large_merkle_proof() {
    let env = Env::default();

    let leaf = BytesN::from_array(&env, &[1u8; 32]);
    let mut proof = vec![&env];

    for i in 2..6 {
        let mut bytes = [0u8; 32];
        bytes[0] = i;
        proof.push_back(BytesN::from_array(&env, &bytes));
    }

    let result = process_proof(&env, &proof, leaf.clone());

    let mut expected = leaf;
    for i in 0..proof.len() {
        expected = commutative_keccak256(&env, expected, proof.get(i).unwrap());
    }

    assert_eq!(result, expected);
}
