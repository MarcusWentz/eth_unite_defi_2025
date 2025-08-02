#![no_std]

use base_escrow::{timelocks::Timelocks, Immutables};
use escrow_factory_interface::EscrowFactoryClient;
use order_interface::Order;
use resolver_interface::ResolverInterface;
use soroban_sdk::{
    contract, contractimpl, log, symbol_short, vec, Address, Bytes, BytesN, Env, Error, IntoVal,
    Symbol, U256,
};

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
        mut immutables: Immutables,
        order: Order,
        signature_r: BytesN<32>,
        signature_vs: BytesN<32>,
        amount: U256,
        taker_traits: U256, // Taker traits = U256
        args: Bytes,
    ) -> Result<Address, Error> {
        let timestamp = U256::from_parts(&env, env.ledger().timestamp(), 0, 0, 0);
        // either we change set_deployed_at to accept pointer to env or we pass env.clone()
        immutables.timelocks =
            Timelocks::set_deployed_at(env.clone(), immutables.timelocks, timestamp);

        let escrow_factory = env
            .storage()
            .instance()
            .get(&ESCROW_FACTORY_ADDRESS)
            .unwrap();
        let escrow_factory_client = EscrowFactoryClient::new(&env, &escrow_factory);

        let address = escrow_factory_client.address_of_escrow_src(&immutables);

        log!(&env, "address: {:?}", address);

        Ok(Address::from_str(&env, ""))
    }

    fn deploy_dst(
        env: Env,
        dst_immutables: Immutables,
        src_cancellation_timestamp: U256,
    ) -> Result<Address, Error> {
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
