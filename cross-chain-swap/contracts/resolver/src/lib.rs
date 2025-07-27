#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Address, Symbol, symbol_short, Error, U256, Bytes, BytesN};
use resolver_interface::ResolverInterface;
use order_interface::Order;
use escrow::Immutables as EscrowImmutables;

#[contract]
pub struct ResolverContract;

const ESCROW_FACTORY_ADDRESS: Symbol = symbol_short!("ESCR_FACT");
const ORDER_MIXIN_ADDRESS: Symbol = symbol_short!("ORDER_MIX");

#[contractimpl]
impl ResolverInterface for ResolverContract {

    fn __constructor(
        env: Env,
        escrow_factory_address: Address,
        order_mixin_address: Address,
    ) {
        env.storage().instance().set(&ESCROW_FACTORY_ADDRESS, &escrow_factory_address);
        env.storage().instance().set(&ORDER_MIXIN_ADDRESS, &order_mixin_address);
    }

    fn get_escrow_factory_address(env: Env) -> Address {
        env.storage().instance().get(&ESCROW_FACTORY_ADDRESS).unwrap()
    }

    fn get_order_mixin_address(env: Env) -> Address {
        env.storage().instance().get(&ORDER_MIXIN_ADDRESS).unwrap()
    }

    fn deploy_src(
        env: Env,
        immutables: EscrowImmutables,
        order: Order,
        signature_r: BytesN<32>,
        signature_vs: BytesN<32>,
        amount: U256,
        taker_traits: U256, // Taker traits = U256
        args: Bytes,
    ) -> Result<Address, Error> {
        Ok(Address::from_str(&env, ""))
    }
    

    fn deploy_dst(
        env: Env,
        dst_immutables: EscrowImmutables,
        src_cancellation_timestamp: U256,
    ) -> Result<Address, Error> {
        Ok(Address::from_str(&env, ""))
    }
}