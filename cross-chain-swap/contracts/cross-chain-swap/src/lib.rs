#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, vec, xdr::ToXdr, Address,
    Bytes, BytesN, Env, String, Symbol, Vec, U256,
};

// CONTRACTS

// Escrow factory, responsible of deploying escrows once a user trades
#[contract]
pub struct EscrowFactory;

// Destination chain escrow contract
#[contract]
pub struct EscrowDst;

// Source chain escrow contract
#[contract]
pub struct EscrowSrc;

// CUSTOM DATA TYPES

// Data for creating the escrow contracts
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Immutables {
    order_hash: BytesN<32>,
    hashlock: BytesN<32>,
    maker: Address,
    taker: Address,
    token: Option<Address>,
    amount: U256,
    safety_deposit: U256,
    timelocks: U256,
}

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
const DST_ESCROW_WASM: Symbol = symbol_short!("DSTWASM");
const SRC_ESCROW_WASM: Symbol = symbol_short!("SRCWASM");

// Contract implementation
#[contractimpl]
impl EscrowFactory {
    // Function for creating destination chain escrow contract
    pub fn create_dst_escrow(
        env: Env,
        dst_immutables: Immutables,
        // Prefixing this with underscore for now, once timelock is implemented we can remove the underscore
        _src_cancellaqtion_timestamp: U256,
    ) -> Result<Address, Error> {
        // First we instantiate the native amount field
        let mut native_amount = dst_immutables.safety_deposit.clone();

        // Then if the requested token is native XML...
        match dst_immutables.token {
            // If so, then we do nothing
            Some(_) => (),
            // Else, we increment the native amount by 1
            None => native_amount = native_amount.add(&U256::from_u32(&env, 1)),
        }

        // Todo here: implement the stellar native timelock: https://github.com/stellar/soroban-examples/tree/v22.0.1/timelock

        // Extract values before moving dst_immutables
        let maker = dst_immutables.maker.clone();
        let hashlock = dst_immutables.hashlock.clone();
        let taker = dst_immutables.taker.clone();

        // Generate salt similar to keccak256(immutables, ESCROW_IMMUTABLES_SIZE)
        // Hash the entire immutables struct to create a deterministic salt
        let salt = env.crypto().sha256(&dst_immutables.to_xdr(&env));

        // Fetching our wasm hash for dst escrow
        let wasm_hash = env
            .storage()
            .instance()
            .get::<_, BytesN<32>>(&DST_ESCROW_WASM)
            .ok_or(Error::EscrowWasmNotAvailable)?;

        // Deploying the contract
        let escrow = env.deployer().with_address(maker, salt).deploy(wasm_hash);

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

mod test;
