use super::base_escrow::{BaseEscrow, Error};
use crate::escrow_factory::timelocks::{Stage, Timelocks};
use escrow::Immutables;
use soroban_sdk::{
    contract, contractimpl, symbol_short, BytesN,
    Env, Symbol,
};

// Source chain escrow contract
#[contract]
pub struct EscrowDst;

#[contractimpl]
impl BaseEscrow for EscrowDst {}

// EVENTS SYMBOLS
const ESCROW_DST: Symbol = symbol_short!("ESC_DST");

// Contract implementation
#[contractimpl]
impl EscrowDst {
    pub fn withdraw(env: Env, secret: BytesN<32>, immutables: Immutables) -> Result<(), Error> {
        Self::only_taker(env.clone(), immutables.clone())?;
        Self::only_after(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::DstWithdrawal,
            ),
        )?;
        Self::only_before(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::DstCancellation,
            ),
        )?;
        Self::withdraw_priv(env, secret, immutables);
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
                Stage::DstPublicWithdrawal,
            ),
        )?;
        Self::only_before(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::DstCancellation,
            ),
        )?;
        Self::withdraw_priv(env, secret, immutables)?;

        Ok(())
    }

    fn cancel(env: Env, immutables: Immutables) -> Result<(), Error> {
        Self::only_taker(env.clone(), immutables.clone())?;
        Self::validate_immutables(env.clone(), immutables.clone())?;
        Self::only_after(
            env.clone(),
            Timelocks::get(env.clone(), immutables.timelocks, Stage::DstCancellation),
        )?;
        Self::uni_transfer(
            env.clone(),
            immutables.token,
            immutables.taker,
            immutables.amount,
        );

        env.events()
            .publish((&ESCROW_DST, symbol_short!("canceled")), ());

        Ok(())
    }

    fn withdraw_priv(env: Env, secret: BytesN<32>, immutables: Immutables) -> Result<(), Error> {
        Self::validate_immutables(env.clone(), immutables.clone())?;
        Self::only_valid_secret(env.clone(), secret.clone(), immutables.clone())?;
        Self::uni_transfer(
            env.clone(),
            immutables.token,
            immutables.maker,
            immutables.amount,
        );
        env.events()
            .publish((&ESCROW_DST, symbol_short!("withdraw")), secret);
        Ok(())
    }
}
