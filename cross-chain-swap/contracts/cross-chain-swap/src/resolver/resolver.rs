#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol, symbol_short};

#[contract]
pub struct ResolverContract;

const ESCROW_FACTORY_ADDRESS: Symbol = symbol_short!("ESCR_FACT");
const ORDER_MIXIN_ADDRESS: Symbol = symbol_short!("ORDER_MIX");

#[contractimpl]
impl ResolverContract {

    pub fn __constructor(
        env: Env,
        escrow_factory_address: Address,
        order_mixin_address: Address,
    ) {
        env.storage().instance().set(&ESCROW_FACTORY_ADDRESS, &escrow_factory_address);
        env.storage().instance().set(&ORDER_MIXIN_ADDRESS, &order_mixin_address);
    }

    pub fn get_escrow_factory_address(env: Env) -> Address {
        env.storage().instance().get(&ESCROW_FACTORY_ADDRESS).unwrap()
    }

    pub fn get_order_mixin_address(env: Env) -> Address {
        env.storage().instance().get(&ORDER_MIXIN_ADDRESS).unwrap()
    }
}