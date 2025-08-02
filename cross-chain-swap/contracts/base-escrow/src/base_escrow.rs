use soroban_sdk::{
    contracterror, symbol_short, token::TokenClient, xdr::ToXdr, Address, BytesN, Env, Symbol, U256,
};

use crate::timelocks::Timelocks;
use crate::Immutables;

// Errors

#[contracterror]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Error {
    InvalidCaller = 1,
    InvalidImmutables = 2,
    InvalidSecret = 3,
    InvalidTime = 4,
    CantUnwrapToken = 5,
    CantUnwrapResqueDelay = 6,
}

// STORAGE SYMBOLS
const RESCUE_DELAY: Symbol = symbol_short!("RES_DEL");
const ACCESS_TOKEN: Symbol = symbol_short!("ACC_TOK");
const XML_ADDRESS: Symbol = symbol_short!("XML_ADD");

// Contract Implementation
pub trait BaseEscrow {
    fn __constructor(env: Env, rescue_delay: u32, access_token: Address) {
        env.storage().instance().set(&RESCUE_DELAY, &rescue_delay);
        env.storage().instance().set(&ACCESS_TOKEN, &access_token);
    }

    // Only take checker&
    fn only_taker(env: Env, immutables: Immutables) -> Result<(), Error> {
        // Make sure that the msg.sender is the correct sender
        if env
            .storage()
            .persistent()
            .get::<_, Address>(&symbol_short!("sender"))
            .unwrap()
            != immutables.taker
        {
            return Err(Error::InvalidCaller);
        };
        Ok(())
    }

    fn validate_immutables(env: Env, immutables: Immutables) -> Result<(), Error> {
        // Extract values before moving immutables
        let maker = immutables.maker.clone();

        // Generate salt similar to keccak256(immutables, ESCROW_IMMUTABLES_SIZE)
        // Hash the entire immutables struct to create a deterministic salt
        let salt = env.crypto().sha256(&immutables.to_xdr(&env));

        // Compute the expected address and compare
        if env.deployer().with_address(maker, salt).deployed_address()
            != env.current_contract_address()
        {
            // If it fails, return error
            return Err(Error::InvalidImmutables);
        }
        Ok(())
    }

    fn only_valid_secret(
        env: Env,
        secret: BytesN<32>,
        immutables: Immutables,
    ) -> Result<(), Error> {
        // Compute expected hashlock and compare
        if env
            .crypto()
            .keccak256(&secret.as_object().to_xdr(&env))
            .to_bytes()
            != immutables.hashlock
        {
            // If not correct, throw an error
            return Err(Error::InvalidSecret);
        }
        Ok(())
    }

    fn only_after(env: Env, start: U256) -> Result<(), Error> {
        if U256::from_u128(&env, env.ledger().timestamp() as u128).lt(&start) {
            return Err(Error::InvalidTime);
        }
        Ok(())
    }

    fn only_before(env: Env, stop: U256) -> Result<(), Error> {
        if U256::from_u128(&env, env.ledger().timestamp() as u128).ge(&stop) {
            return Err(Error::InvalidTime);
        }
        Ok(())
    }

    fn only_acess_token_holder(env: Env) -> Result<(), Error> {
        // Make sure that msg.sender holds any access tokens
        if TokenClient::new(
            &env,
            &env.storage()
                .instance()
                .get::<_, Address>(&ACCESS_TOKEN)
                .ok_or(Error::CantUnwrapToken)?,
        )
        .balance(
            &env.storage()
                .persistent()
                .get(&symbol_short!("sender"))
                .unwrap(),
        ) == 0
        {
            return Err(Error::InvalidCaller);
        }
        Ok(())
    }

    fn rescue_funds(
        env: Env,
        token: Address,
        amount: i128,
        immutables: Immutables,
    ) -> Result<(), Error> {
        Self::only_taker(env.clone(), immutables.clone())?;
        Self::validate_immutables(env.clone(), immutables.clone())?;
        Self::only_after(
            env.clone(),
            Timelocks::rescue_start(
                immutables.timelocks,
                env.storage()
                    .instance()
                    .get(&RESCUE_DELAY)
                    .ok_or(Error::CantUnwrapResqueDelay)?,
            ),
        )?;

        Self::uni_transfer(
            env.clone(),
            token,
            env.storage()
                .persistent()
                .get(&symbol_short!("sender"))
                .unwrap(),
            amount,
        );

        Ok(())
    }

    fn uni_transfer(env: Env, token: Address, to: Address, amount: i128) {
        TokenClient::new(&env, &token).transfer(&env.current_contract_address(), &to, &amount);
    }

    fn xlm_transfer(env: Env, to: Address, amount: i128) {
        let xlm = env
            .storage()
            .instance()
            .get::<_, Address>(&XML_ADDRESS)
            .unwrap();
        TokenClient::new(&env, &xlm).transfer(&env.current_contract_address(), &to, &amount);
    }
}
