use suse super::base_escrow::{BaseEscrow, Error};
use crate::escrow_factory::timelocks::{Stage, Timelocks};
use escrow::Immutables;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, xdr::ToXdr, Address, BytesN,
    Env, Symbol, U256,
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
impl EscrowDst {

}
