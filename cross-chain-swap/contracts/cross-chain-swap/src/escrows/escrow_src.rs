use super::base_escrow::{BaseEscrow, Error};
use crate::escrow_factory::timelocks::{Stage, Timelocks};
use escrow::Immutables;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token::TokenClient,
    xdr::ToXdr, Address, BytesN, Env, Symbol, U256,
};

// Source chain escrow contract
#[contract]
pub struct EscrowSrc;

#[contractimpl]
impl BaseEscrow for EscrowSrc {}

// EVENTS SYMBOLS
const ESCROW_SRC: Symbol = symbol_short!("ESC_SRC");

// Contract implementation
#[contractimpl]
impl EscrowSrc {
    fn withdraw_to(
        env: Env,
        secret: BytesN<32>,
        target: Address,
        immutables: Immutables,
    ) -> Result<(), Error> {
        Self::validate_immutables(env.clone(), immutables.clone())?;
        Self::only_valid_secret(env.clone(), secret.clone(), immutables.clone())?;
        TokenClient::new(&env, &immutables.token).transfer(
            &env.current_contract_address(),
            &target,
            &immutables.amount,
        );

        Self::xlm_transfer(
            env.clone(),
            env.storage()
                .persistent()
                .get(&symbol_short!("sender"))
                .unwrap(),
            immutables.safety_deposit,
        );

        env.events()
            .publish((&ESCROW_SRC, symbol_short!("withdraw")), secret);

        Ok(())
    }

    fn cancel(env: Env, immutables: Immutables) -> Result<(), Error> {
        Self::validate_immutables(env.clone(), immutables.clone())?;
        TokenClient::new(&env, &immutables.token).transfer(
            &env.current_contract_address(),
            &immutables.maker,
            &immutables.amount,
        );
        Self::xlm_transfer(
            env.clone(),
            env.storage()
                .persistent()
                .get(&symbol_short!("sender"))
                .unwrap(),
            immutables.safety_deposit,
        );

        env.events()
            .publish((&ESCROW_SRC, symbol_short!("canceled")), ());

        Ok(())
    }
}
