#![no_std]
use soroban_sdk::{contractclient, Env, Address, BytesN, U256, Bytes, Error};

use order_interface::Order;
use escrow::Immutables as EscrowImmutables;

/// Interface for the sample implementation of a Resolver contract for cross-chain swap.
#[contractclient(name = "ResolverInterfaceClient")]
pub trait ResolverInterface {

    fn __constructor(
        env: Env,
        escrow_factory_address: Address,
        order_mixin_address: Address,
    );

    fn get_escrow_factory_address(env: Env) -> Address;

    fn get_order_mixin_address(env: Env) -> Address;

     /// Deploys a new escrow contract for maker on the source chain
    /// 
    /// # Arguments
    /// * `immutables` - The immutables of the escrow contract used in deployment
    /// * `order` - Order quote to fill
    /// * `signature_r` - R component of signature
    /// * `signature_vs` - VS component of signature  
    /// * `amount` - Taker amount to fill
    /// * `taker_traits` - Taker execution traits
    /// * `args` - Additional arguments for the taker
    fn deploy_src(
        env: Env,
        immutables: EscrowImmutables,
        order: Order,
        signature_r: BytesN<32>,
        signature_vs: BytesN<32>,
        amount: U256,
        taker_traits: U256, // Taker traits = U256
        args: Bytes,
    ) -> Result<Address, Error>; // original function does not and external return

    /// Deploys a new escrow contract for taker on the destination chain
    /// 
    /// # Arguments
    /// * `dst_immutables` - The immutables of the escrow contract used in deployment
    /// * `src_cancellation_timestamp` - The start of the cancellation period for the source chain
    fn deploy_dst(
        env: Env,
        dst_immutables: EscrowImmutables,
        src_cancellation_timestamp: U256,
    ) -> Result<Address, Error>; // original function does not and external return
}
