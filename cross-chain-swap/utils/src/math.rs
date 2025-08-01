use soroban_sdk::{Bytes, Env, U256};

pub fn max_num<'a>(a: &'a U256, b: &'a U256) -> &'a U256 {
    a.max(b)
}

pub fn min_num<'a>(a: &'a U256, b: &'a U256) -> &'a U256 {
    a.min(b)
}

pub fn bitand(env: &Env, a: U256, b: U256) -> U256 {
    let a_bytes = a.to_be_bytes();
    let b_bytes = b.to_be_bytes();

    let mut result = Bytes::from_array(&env, &[0; 32]);

    for i in 0..32 {
        let byte_result = a_bytes.get(i).unwrap_or(0) & b_bytes.get(i).unwrap_or(0);
        result.set(i, byte_result);
    }

    return U256::from_be_bytes(&env, &result);
}
