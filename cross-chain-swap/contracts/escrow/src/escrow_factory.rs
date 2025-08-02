use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, xdr::ToXdr, Address, BytesN,
    Env, Symbol, U256,
};

use base_escrow::timelocks::{Stage, Timelocks};
use base_escrow::Immutables;
use escrow_factory_interface::EscrowFactoryInterface;

// CONTRACTS

// Escrow factory, responsible of deploying escrows once a user trades
#[contract]
pub struct EscrowFactory;

// CUSTOM DATA TYPES

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowType {
    Destination,
    Source,
}

// Errors

#[contracterror]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Error {
    EscrowWasmNotAvailable = 1,
    InsufficientEscrowBalance = 2,
    InvalidCreationTime = 3,
}

// Events data types
#[contracttype]
pub struct DstEscrowCreated {
    escrow_address: Address,
    hashlock: BytesN<32>,
    taker: Address,
}

// EVENTS SYMBOLS
const ESCROW_CREATED: Symbol = symbol_short!("ESCR");

// STORAGE SYMBOLS
const DST_ESCROW_WASM: Symbol = symbol_short!("DST_WASM");
const SRC_ESCROW_WASM: Symbol = symbol_short!("SRC_WASM");
const XLM_ADDRESS: Symbol = symbol_short!("XLM_ADD");

// Contract implementation
#[contractimpl]
impl EscrowFactoryInterface for EscrowFactory {
    fn __constructor(
        env: Env,
        escrow_dst_wasm_hash: BytesN<32>,
        escrow_src_wasm_hash: BytesN<32>,
        xlm_address: Address,
    ) {
        env.storage()
            .instance()
            .set(&DST_ESCROW_WASM, &escrow_dst_wasm_hash);
        env.storage()
            .instance()
            .set(&SRC_ESCROW_WASM, &escrow_src_wasm_hash);
        env.storage().instance().set(&XLM_ADDRESS, &xlm_address);
    }

    // Function for creating destination chain escrow contract
    fn create_dst_escrow(
        env: Env,
        // dst_immutables is modified later, so #[allow(unused_mut)] is used to hide the warning that it doesn't need mut when it does.
        mut dst_immutables: Immutables,
        src_cancellation_timestamp: U256,
        native_token_lock_value: u128,
    ) -> Address {
        // First we instantiate the native amount field
        let mut native_amount = dst_immutables.safety_deposit.clone();

        // Get the native token address for comparison
        let xlm_address = env
            .storage()
            .instance()
            .get::<_, Address>(&XLM_ADDRESS)
            .unwrap();

        // Then if the requested token is native XLM...
        if xlm_address == dst_immutables.token {
            // We increment the native amount by the token amount
            native_amount = native_amount + dst_immutables.amount;
        }

        // Convert both values to U256 for comparison
        let native_amount_u256 = U256::from_u128(&env, native_amount as u128);
        let provided_native_amount = U256::from_u128(&env, native_token_lock_value);

        // Making sure native amount exactly matches the provided value (like Solidity's msg.value check)
        if native_amount_u256 != provided_native_amount {
            panic!("InsufficientEscrowBalance");
            // panic!("debug native_amount_u256: {:?}", native_amount_u256);
            // panic!("debug provided_native_amount: {:?}", provided_native_amount);
        }

        // Swap out deployment time
        dst_immutables.timelocks = Timelocks::set_deployed_at(
            env.clone(),
            dst_immutables.timelocks,
            U256::from_u128(&env, env.ledger().timestamp() as u128),
        );

        // Make sure that the deployment time is valid
        if Timelocks::get(
            env.clone(),
            dst_immutables.timelocks.clone(),
            Stage::DstCancellation,
        )
        .gt(&src_cancellation_timestamp)
        {
            panic!("InvalidCreationTime");
        }

        // Extract values before moving dst_immutables
        let maker = dst_immutables.maker.clone();
        let hashlock = dst_immutables.hashlock.clone();
        let taker = dst_immutables.taker.clone();
        let token = dst_immutables.token.clone();
        let amount = dst_immutables.amount.clone();

        // Generate salt similar to keccak256(immutables, ESCROW_IMMUTABLES_SIZE)
        // Hash the entire immutables struct to create a deterministic salt
        let salt = env.crypto().keccak256(&dst_immutables.to_xdr(&env));

        // Fetching our wasm hash for dst escrow
        let wasm_hash = env
            .storage()
            .instance()
            .get::<_, BytesN<32>>(&DST_ESCROW_WASM);

        if wasm_hash.is_none() {
            panic!("EscrowWasmNotAvailable");
        }

        // Require authorization from the maker
        maker.require_auth();

        // Deploying the contract
        let escrow = env
            .deployer()
            .with_address(maker.clone(), salt)
            .deploy_v2(wasm_hash.unwrap(), ());

        // Transfer tokens to escrow (works for both XLM and other tokens in Stellar)
        // This mirrors the Solidity: IERC20(token).safeTransferFrom(msg.sender, escrow, amount)
        let amount_signed: i128 = amount
            .clone()
            .try_into()
            .expect("u128 value too large for i128");
        let token_client = TokenClient::new(&env, &token);
        token_client.transfer(&maker, &escrow, &amount_signed);

        // We emit the event
        env.events().publish(
            (&ESCROW_CREATED, symbol_short!("dst")),
            DstEscrowCreated {
                escrow_address: escrow.clone(),
                hashlock,
                taker,
            },
        );

        // Return the escrow contract address
        return escrow;
    }

    fn address_of_escrow_src(env: Env, immutables: Immutables) -> Address {
        // Extract maker before moving immutables
        let maker = immutables.maker.clone();
        let salt = env.crypto().keccak256(&immutables.to_xdr(&env));
        env.deployer().with_address(maker, salt).deployed_address()
    }
}
