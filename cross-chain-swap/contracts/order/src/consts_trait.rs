use soroban_sdk::{contract, Address, Env, U256};

pub trait ConstTrait {
    const EXPIRATION_OFFSET: u32 = 80;
    const NONCE_OR_EPOCH_OFFSET: u32 = 120;
    const SERIES_OFFSET: u32 = 160;
    const ARGS_EXTENSION_LENGTH_OFFSET: u32 = 224;
    const ARGS_INTERACTION_LENGTH_OFFSET: u32 = 200;

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

    fn args_has_target(env: Env) -> U256 {
        U256::from_u32(&env, 1).shl(251)
    }
}
