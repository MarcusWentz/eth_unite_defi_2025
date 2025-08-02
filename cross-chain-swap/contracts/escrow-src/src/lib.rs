#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, token::TokenClient, Address, Env, BytesN, Symbol};

use base_escrow::{base_escrow::BaseEscrow, Immutables};
use base_escrow::timelocks::{Stage, Timelocks};

#[contract]
pub struct EscrowSrc;

#[contractimpl]
impl BaseEscrow for EscrowSrc {}

// EVENTS SYMBOLS
const ESCROW_SRC: Symbol = symbol_short!("ESC_SRC");

#[contractimpl]
impl EscrowSrc {
    fn withdraw(env: Env, secret: BytesN<32>, immutables: Immutables) {
        let res = Self::only_taker(env.clone(), immutables.clone());
        if let Err(e) = res {
            panic!("Not a taker");
        }
        let res = Self::only_after(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::SrcWithdrawal,
            ),
        );
        if let Err(e) = res {
            panic!("Not after withdrawal");
        }

        let res = Self::only_before(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::SrcCancellation,
            ),
        );
        if let Err(e) = res {
            panic!("Not before withdrawal");
        }

        let res = Self::withdraw_to_priv(
            env.clone(),
            secret,
            env.storage()
                .persistent()
                .get(&symbol_short!("sender"))
                .unwrap(),
            immutables,
        );
    }

    fn withdraw_to_priv(
        env: Env,
        secret: BytesN<32>,
        target: Address,
        immutables: Immutables,
    ) {
        let res = Self::validate_immutables(env.clone(), immutables.clone());
        if let Err(e) = res {
            panic!("Invalid immutables");
        }
        let res = Self::only_valid_secret(env.clone(), secret.clone(), immutables.clone());
        if let Err(e) = res {
            panic!("Invalid secret");
        }
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
    }
}
