use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, 
    xdr::ToXdr, 
    Address,
    BytesN, 
    Env, 
    Symbol, 
    U256, 
    Vec, 
    Val, 
};

use super::timelocks::{Stage, Timelocks};
use escrow::Immutables;

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
const XML_ADDRESS: Symbol = symbol_short!("XML_ADD");

// Contract implementation
#[contractimpl]
impl EscrowFactory {
    // Function for creating destination chain escrow contract
    pub fn create_dst_escrow(
        env: Env,
        // dst_immutables is modified later, so #[allow(unused_mut)] is used to hide the warning that it doesn't need mut when it does.
        #[allow(unused_mut)] mut dst_immutables: Immutables,
        // Prefixing this with underscore for now, once timelock is implemented we can remove the underscore
        src_cancellaqtion_timestamp: U256,
    ) -> Result<Address, Error> {
        // First we instantiate the native amount field
        let mut native_amount = dst_immutables.safety_deposit.clone();

        // Then if the requested token is native XML...
        if env
            .storage()
            .instance()
            .get::<_, Address>(&XML_ADDRESS)
            .unwrap()
            == dst_immutables.token
        {
            // We increment the native amount by 1
            native_amount = native_amount + dst_immutables.amount;
        }

        // fetching the msg.value
        let msg_value: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("value"))
            .unwrap();

        // Making sure native amount does not excede the msg.value
        if native_amount.lt(&msg_value) {
            return Err(Error::InsufficientEscrowBalance);
        };

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
        .gt(&src_cancellaqtion_timestamp)
        {
            return Err(Error::InvalidCreationTime);
        };

        // Extract values before moving dst_immutables
        let maker = dst_immutables.maker.clone();
        let hashlock = dst_immutables.hashlock.clone();
        let taker = dst_immutables.taker.clone();

        // Generate salt similar to keccak256(immutables, ESCROW_IMMUTABLES_SIZE)
        // Hash the entire immutables struct to create a deterministic salt
        let salt = env.crypto().keccak256(&dst_immutables.to_xdr(&env));

        // Fetching our wasm hash for dst escrow
        let wasm_hash = env
            .storage()
            .instance()
            .get::<_, BytesN<32>>(&DST_ESCROW_WASM)
            .ok_or(Error::EscrowWasmNotAvailable)?;

        // Deploying the contract
        let escrow = env
            .deployer()
            .with_address(maker, salt)
            .deploy_v2(wasm_hash, ());

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
        Ok(escrow)
    }

    // Function for storing the different kinds of escrow contract wasm
    pub fn store_escrow_wasm(env: Env, contract_wasm: BytesN<32>, escrow_type: EscrowType) {
        // Store into different storage allocations depending on the escrow type
        match escrow_type {
            EscrowType::Destination => env
                .storage()
                .instance()
                .set(&DST_ESCROW_WASM, &contract_wasm),
            EscrowType::Source => env
                .storage()
                .instance()
                .set(&SRC_ESCROW_WASM, &contract_wasm),
        }
    }
}
