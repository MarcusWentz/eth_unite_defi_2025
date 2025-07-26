use soroban_sdk::{Bytes, BytesN, Env, Vec};

pub fn concat_bytes(env: &Env, a: BytesN<32>, b: BytesN<32>) -> BytesN<32> {
    let mut combined = Bytes::new(env);
    combined.extend_from_array(&a.to_array());
    combined.extend_from_array(&b.to_array());
    env.crypto().keccak256(&combined).into()
}

pub fn commutative_keccak256(env: &Env, a: BytesN<32>, b: BytesN<32>) -> BytesN<32> {
    if a < b {
        concat_bytes(env, a, b)
    } else {
        concat_bytes(env, b, a)
    }
}

pub fn process_proof(env: &Env, proof: &Vec<BytesN<32>>, leaf: BytesN<32>) -> BytesN<32> {
    let mut computed_hash = leaf;

    for i in 0..proof.len() {
        computed_hash = commutative_keccak256(env, computed_hash, proof.get(i).unwrap());
    }

    computed_hash
}

pub struct MerkleProof {
    pub root: BytesN<32>,
    pub leaf: BytesN<32>,
}

impl MerkleProof {
    pub fn verify(env: &Env, proof: &Vec<BytesN<32>>, root: BytesN<32>, leaf: BytesN<32>) -> bool {
        process_proof(env, proof, leaf) == root
    }
}
