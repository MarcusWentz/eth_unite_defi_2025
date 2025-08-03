#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, BytesN, Env, Symbol};

use base_escrow::timelocks::{Stage, Timelocks};
use base_escrow::{base_escrow::BaseEscrow, Immutables};

#[contract]
pub struct EscrowDst;

#[contractimpl]
impl BaseEscrow for EscrowDst {}

#[allow(dead_code)]
const ESCROW_DST: Symbol = symbol_short!("ESC_DST");

#[contractimpl]
impl EscrowDst {
    #[allow(dead_code)]
    fn withdraw(env: Env, secret: BytesN<32>, immutables: Immutables) {
        if let Err(_e) = Self::only_taker(env.clone(), immutables.clone()) {
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
        if let Err(_e) = res {
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
        if let Err(_e) = res {
            panic!("Not after withdrawal");
        }

        Self::withdraw_priv(env, secret, immutables);
    }

    #[allow(dead_code)]
    fn public_withdraw(env: Env, secret: BytesN<32>, immutables: Immutables) {
        let res = Self::only_acess_token_holder(env.clone());
        if let Err(_e) = res {
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
        if let Err(_e) = res {
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
        if let Err(_e) = res {
            panic!("Not before public withdrawal");
        }
        Self::withdraw_priv(env, secret, immutables);
    }

    #[allow(dead_code)]
    fn cancel(env: Env, immutables: Immutables) {
        let res = Self::only_taker(env.clone(), immutables.clone());
        if let Err(_e) = res {
            panic!("Not a taker");
        }
        let res = Self::validate_immutables(env.clone(), immutables.clone());
        if let Err(_e) = res {
            panic!("Invalid immutables");
        }
        let res = Self::only_after(
            env.clone(),
            Timelocks::get(env.clone(), immutables.timelocks, Stage::DstCancellation),
        );
        if let Err(_e) = res {
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

    #[allow(dead_code)]
    fn withdraw_priv(env: Env, secret: BytesN<32>, immutables: Immutables) {
        let res = Self::validate_immutables(env.clone(), immutables.clone());
        if let Err(_e) = res {
            panic!("Invalid immutables");
        }
        let res = Self::only_valid_secret(env.clone(), secret.clone(), immutables.clone());
        if let Err(_e) = res {
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
