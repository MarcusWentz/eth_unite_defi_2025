use super::base_escrow::{BaseEscrow, Error};
use escrow::{
    timelocks::{Stage, Timelocks},
    Immutables,
};
use soroban_sdk::{
    contract, contractimpl, symbol_short, token::TokenClient, Address, BytesN, Env, Symbol,
};

// Source chain escrow contract
#[contract]
pub struct EscrowSrc;

#[contractimpl]
impl BaseEscrow for EscrowSrc {}

// EVENTS SYMBOLS
const ESCROW_SRC: Symbol = symbol_short!("ESC_SRC");

// Contract implementation
impl EscrowSrc {
    pub fn withdraw(env: Env, secret: BytesN<32>, immutables: Immutables) -> Result<(), Error> {
        Self::only_taker(env.clone(), immutables.clone())?;
        Self::only_after(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::SrcWithdrawal,
            ),
        )?;
        Self::only_before(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::SrcCancellation,
            ),
        )?;
        Self::withdraw_to_priv(
            env.clone(),
            secret,
            env.storage()
                .persistent()
                .get(&symbol_short!("sender"))
                .unwrap(),
            immutables,
        )?;

        Ok(())
    }

    pub fn withdraw_to(
        env: Env,
        secret: BytesN<32>,
        target: Address,
        immutables: Immutables,
    ) -> Result<(), Error> {
        Self::only_taker(env.clone(), immutables.clone())?;
        Self::only_after(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::SrcWithdrawal,
            ),
        )?;
        Self::only_before(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::SrcCancellation,
            ),
        )?;
        Self::withdraw_to_priv(env, secret, target, immutables)?;

        Ok(())
    }

    pub fn public_withdraw(
        env: Env,
        secret: BytesN<32>,
        immutables: Immutables,
    ) -> Result<(), Error> {
        Self::only_acess_token_holder(env.clone())?;
        Self::only_after(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::SrcPublicWithdrawal,
            ),
        )?;
        Self::only_before(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::SrcCancellation,
            ),
        )?;
        Self::withdraw_to_priv(env, secret, immutables.taker.clone(), immutables)?;

        Ok(())
    }

    pub fn cancel(env: Env, immutables: Immutables) -> Result<(), Error> {
        Self::only_taker(env.clone(), immutables.clone())?;
        Self::only_after(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::SrcCancellation,
            ),
        )?;
        Self::cancel_priv(env, immutables)?;
        Ok(())
    }

    pub fn public_cancel(env: Env, immutables: Immutables) -> Result<(), Error> {
        Self::only_acess_token_holder(env.clone())?;
        Self::only_after(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::SrcPublicCancellation,
            ),
        )?;
        Self::cancel_priv(env, immutables)?;
        Ok(())
    }

    fn withdraw_to_priv(
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
            &immutables.amount.try_into().unwrap(),
        );

        Self::xlm_transfer(
            env.clone(),
            env.storage()
                .persistent()
                .get(&symbol_short!("sender"))
                .unwrap(),
            immutables.safety_deposit.try_into().unwrap(),
        );

        env.events()
            .publish((&ESCROW_SRC, symbol_short!("withdraw")), secret);

        Ok(())
    }

    fn cancel_priv(env: Env, immutables: Immutables) -> Result<(), Error> {
        Self::validate_immutables(env.clone(), immutables.clone())?;
        TokenClient::new(&env, &immutables.token).transfer(
            &env.current_contract_address(),
            &immutables.maker,
            &immutables.amount.try_into().unwrap(),
        );
        Self::xlm_transfer(
            env.clone(),
            env.storage()
                .persistent()
                .get(&symbol_short!("sender"))
                .unwrap(),
            immutables.safety_deposit.try_into().unwrap(),
        );

        env.events()
            .publish((&ESCROW_SRC, symbol_short!("canceled")), ());

        Ok(())
    }
}
