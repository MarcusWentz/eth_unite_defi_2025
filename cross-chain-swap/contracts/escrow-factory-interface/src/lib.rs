#![no_std]
use soroban_sdk::{contractclient, Address, BytesN, Env, U256};
use base_escrow::Immutables;

#[contractclient(name = "EscrowFactoryClient")]
pub trait EscrowFactoryInterface {
    fn __constructor(env: Env, escrow_dst_wasm_hash: BytesN<32>, escrow_src_wasm_hash: BytesN<32>, xlm_address: Address);

    fn create_dst_escrow(env: Env, immutables: Immutables, src_cancellation_timestamp: U256, native_token_lock_value: u128) -> Address;

    fn address_of_escrow_src(env: Env, immutables: Immutables) -> Address;
}
