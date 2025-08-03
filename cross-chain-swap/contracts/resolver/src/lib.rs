#![no_std]

use base_escrow::{timelocks::Timelocks, Immutables};
use escrow_factory_interface::EscrowFactoryClient;
use order_interface::{AuctionDetails, Order, OrderClient};
use resolver_interface::ResolverInterface;
use soroban_sdk::{
    contract, contractimpl, log, symbol_short, token::TokenClient, vec, xdr::{FromXdr, ToXdr}, Address, Bytes, BytesN, Env, Error, IntoVal, Symbol, U256
};
use utils::math;

#[contract]
pub struct ResolverContract;

const ESCROW_FACTORY_ADDRESS: Symbol = symbol_short!("ESCR_FACT");
const ORDER_MIXIN_ADDRESS: Symbol = symbol_short!("ORDER_MIX");

#[contractimpl]
impl ResolverInterface for ResolverContract {
    fn __constructor(env: Env, escrow_factory_address: Address, order_mixin_address: Address) {
        env.storage()
            .instance()
            .set(&ESCROW_FACTORY_ADDRESS, &escrow_factory_address);
        env.storage()
            .instance()
            .set(&ORDER_MIXIN_ADDRESS, &order_mixin_address);
    }

    fn get_escrow_factory_address(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&ESCROW_FACTORY_ADDRESS)
            .unwrap()
    }

    fn get_order_mixin_address(env: Env) -> Address {
        env.storage().instance().get(&ORDER_MIXIN_ADDRESS).unwrap()
    }

    fn deploy_src(
        env: Env,
        immutables: Immutables,
        order: Order,
        signature_r: BytesN<32>,
        signature_vs: BytesN<32>,
        amount: U256,
        taker_traits: U256, // Taker traits = U256
        args: Bytes,
    ) -> Address {
        let mut immutables_mem = immutables.clone();
        let timestamp = U256::from_u128(&env, env.ledger().timestamp().try_into().unwrap());
        // either we change set_deployed_at to accept pointer to env or we pass env.clone()
        immutables_mem.timelocks =
            Timelocks::set_deployed_at(env.clone(), immutables_mem.timelocks, timestamp);

        let escrow_factory = env
            .storage()
            .instance()
            .get(&ESCROW_FACTORY_ADDRESS)
            .unwrap();
        let escrow_factory_client = EscrowFactoryClient::new(&env, &escrow_factory);

        let address = escrow_factory_client.address_of_escrow_src(&immutables_mem);

        let token_client = TokenClient::new(&env, &order.maker_asset);

        let safty_deposit_amount = immutables_mem.safety_deposit.try_into().unwrap();
        let transfer_result = token_client.try_transfer(&order.maker, &address, &safty_deposit_amount);
        if transfer_result.is_err() {
            panic!("Failed to transfer safety deposit");
        }

        let updated_taker_traits = math::bit_or(&env, taker_traits, U256::from_u32(&env, 1).shl(251));

        let mut args_mem = args.clone();
        let address_bytes = Bytes::from_xdr(&env, &address.clone().to_xdr(&env));
        if address_bytes.is_err() {
            panic!("Failed to convert address to bytes");
        }
        args_mem.append(&address_bytes.unwrap());

        let order_mixin = env.storage().instance().get(&ORDER_MIXIN_ADDRESS).unwrap();
        let order_mixin_client = OrderClient::new(&env, &order_mixin);

        let auction_details = AuctionDetails {
            auction_start_time: U256::from_u32(&env, 0),
            taking_amount_start: U256::from_u32(&env, 0),
            taking_amount_end: U256::from_u32(&env, 0),
        };

        order_mixin_client.fill_order_args(
            &order,
            &signature_r,
            &signature_vs,
            &amount,
            &updated_taker_traits,
            &args_mem,
            &auction_details,
        );

        return address.clone();
    }

    fn deploy_dst(
        env: Env,
        dst_immutables: Immutables,
        src_cancellation_timestamp: U256,
    ) -> Address {
        // create_dst_escrow(&env, dst_immutables, src_cancellation_timestamp)
        let escrow_factory_address = Self::get_escrow_factory_address(env.clone());

        // Call the escrow factory contract to create the destination escrow
        env.invoke_contract(
            &escrow_factory_address,
            &Symbol::new(&env, "create_dst_escrow"),
            vec![
                &env,
                dst_immutables.into_val(&env),
                src_cancellation_timestamp.into_val(&env),
            ],
        )
    }
}

mod test;
