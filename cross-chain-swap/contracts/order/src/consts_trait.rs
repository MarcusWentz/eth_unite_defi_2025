use soroban_sdk::{contract, Address, Bytes, Env, U256};

pub trait ConstTrait {
    const EXPIRATION_OFFSET: u32 = 80;
    const NONCE_OR_EPOCH_OFFSET: u32 = 120;
    const SERIES_OFFSET: u32 = 160;
    const ARGS_EXTENSION_LENGTH_OFFSET: u32 = 224;
    const ARGS_EXTENSION_LENGTH_MASK: u32 = 0xffffff;
    const ARGS_INTERACTION_LENGTH_OFFSET: u32 = 200;
    const ARGS_INTERACTION_LENGTH_MASK: u32 = 0xffffff;

    // Funcs replacing consts

    fn allowed_sender_mask(env: Env) -> U256 {
        U256::from_u32(&env, 1)
            .shl(80)
            .sub(&U256::from_u32(&env, 1))
    }

    fn expiration_mask(env: Env) -> U256 {
        U256::from_u32(&env, 1)
            .shl(40)
            .sub(&U256::from_u32(&env, 1))
    }

    fn nonce_or_epoch_mask(env: Env) -> U256 {
        U256::from_u32(&env, 1)
            .shl(40)
            .sub(&U256::from_u32(&env, 1))
    }

    fn series_mask(env: Env) -> U256 {
        U256::from_u32(&env, 1)
            .shl(40)
            .sub(&U256::from_u32(&env, 1))
    }

    fn no_partial_fills_flag(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(255)
    }

    fn allow_multiple_fills_flag(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(254)
    }

    fn pre_interaction_call_flag(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(252)
    }

    fn post_interaction_call_flag(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(251)
    }

    fn need_check_epoch_manager_flag(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(250)
    }

    fn has_extension_flag(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(249)
    }

    fn use_permit2_maker_flag(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(248)
    }

    fn unwrap_weth_maker_flag(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(248)
    }

    fn maker_amount_flag(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(255)
    }

    fn unwrap_weth_taker_flag(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(254)
    }

    fn skip_order_permit_flag(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(253)
    }

    fn use_permit2_taker_flag(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(252)
    }

    fn args_has_target_const(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(251)
    }

    fn amount_mask(env: Env) -> U256 {
        // 0x000000000000000000ffffffffffffffffffffffffffffffffffffffffffffff
        let bytes = Bytes::from_array(
            &env,
            &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
            ],
        );
        U256::from_be_bytes(&env, &bytes)
    }

    // Funcs shared

    // Helper function to check if a specific bit flag is set
    fn check_flag(env: Env, taker_traits: U256, bit_position: U256) -> bool {
        // Create a mask with the bit set at the specified position
        let mask = U256::from_u32(&env, 1).shl(bit_position.to_u128().unwrap() as u32);
        u256_bitwise_and(&env, &taker_traits, &mask) != U256::from_u32(&env, 0)
    }
}

/// More efficient bitwise AND implementation
pub fn u256_bitwise_and(env: &Env, a: &U256, b: &U256) -> U256 {
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
