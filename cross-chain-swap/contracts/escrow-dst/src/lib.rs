#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, BytesN, Symbol};

use base_escrow::{base_escrow::BaseEscrow, Immutables};
use base_escrow::timelocks::{Stage, Timelocks};


#[contract]
pub struct EscrowDst;

#[contractimpl]
impl BaseEscrow for EscrowDst {}

const ESCROW_DST: Symbol = symbol_short!("ESC_DST");

#[contractimpl]
impl EscrowDst {
    fn withdraw(env: Env, secret: BytesN<32>, immutables: Immutables) {
        if let Err(e) = Self::only_taker(env.clone(), immutables.clone()) {
            panic!("Not a taker");
        }
        let res = Self::only_after(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::DstWithdrawal,
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
                Stage::DstCancellation,
            ),
        );
        if let Err(e) = res {
            panic!("Not after withdrawal");
        }

        Self::withdraw_priv(env, secret, immutables);
    }

    fn public_withdraw(
        env: Env,
        secret: BytesN<32>,
        immutables: Immutables,
    ) {
        let res = Self::only_acess_token_holder(env.clone());
        if let Err(e) = res {
            panic!("Not a access token holder");
        }
        let res = Self::only_after(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::DstPublicWithdrawal,
            ),
        );
        if let Err(e) = res {
            panic!("Not after public withdrawal");
        }
        let res = Self::only_before(
            env.clone(),
            Timelocks::get(
                env.clone(),
                immutables.timelocks.clone(),
                Stage::DstCancellation,
            ),
        );
        if let Err(e) = res {
            panic!("Not before public withdrawal");
        }
        Self::withdraw_priv(env, secret, immutables);
    }

    fn cancel(env: Env, immutables: Immutables) {
        let res = Self::only_taker(env.clone(), immutables.clone());
        if let Err(e) = res {
            panic!("Not a taker");
        }
        let res = Self::validate_immutables(env.clone(), immutables.clone());
        if let Err(e) = res {
            panic!("Invalid immutables");
        }
        let res = Self::only_after(
            env.clone(),
            Timelocks::get(env.clone(), immutables.timelocks, Stage::DstCancellation),
        );
        if let Err(e) = res {
            panic!("Not after cancellation");
        }
        Self::uni_transfer(
            env.clone(),
            immutables.token,
            immutables.taker,
            immutables.amount as i128,
        );

        env.events()
            .publish((&ESCROW_DST, symbol_short!("canceled")), ());
    }

    fn withdraw_priv(env: Env, secret: BytesN<32>, immutables: Immutables) {
        let res = Self::validate_immutables(env.clone(), immutables.clone());
        if let Err(e) = res {
            panic!("Invalid immutables");
        }
        let res = Self::only_valid_secret(env.clone(), secret.clone(), immutables.clone());
        if let Err(e) = res {
            panic!("Invalid secret");
        }
        Self::uni_transfer(
            env.clone(),
            immutables.token,
            immutables.maker,
            immutables.amount as i128,
        );
        env.events()
            .publish((&ESCROW_DST, symbol_short!("withdraw")), secret);
    }
    
}
