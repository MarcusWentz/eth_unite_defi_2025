use super::base_escrow::BaseEscrow;
use escrow::Immutables;
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, xdr::ToXdr, Address, BytesN,
    Env, Symbol, U256,
};

// Source chain escrow contract
#[contract]
pub struct EscrowDst;

#[contractimpl]
impl BaseEscrow for EscrowDst {
    fn withdraw(secret: BytesN<32>, immutables: Immutables)
}
