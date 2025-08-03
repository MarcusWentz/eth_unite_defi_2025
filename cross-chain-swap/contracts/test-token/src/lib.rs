#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol,
};

#[cfg(test)]
mod test_authentication;

#[contract]
pub struct TestToken;

const DECIMALS: u32 = 18;
#[allow(dead_code)]
const NAME: Symbol = symbol_short!("TestTok");
#[allow(dead_code)]
const SYMBOL: Symbol = symbol_short!("TT");

// Storage keys
const TOTAL_SUPPLY: Symbol = symbol_short!("TOTAL");
const BALANCE: Symbol = symbol_short!("BALANCE");
const ALLOWANCE: Symbol = symbol_short!("ALLOW");

#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[contractimpl]
impl TestToken {
    pub fn __constructor(env: Env, admin: Address) {
        // Set initial supply to 1 billion tokens
        let total_supply: i128 = 1_000_000_000 * 10_i128.pow(DECIMALS);

        // Give all tokens to admin initially
        env.storage().persistent().set(&TOTAL_SUPPLY, &total_supply);
        env.storage()
            .persistent()
            .set(&(BALANCE, admin.clone()), &total_supply);
    }

    pub fn decimals(_env: Env) -> u32 {
        DECIMALS
    }

    pub fn name(_env: Env) -> String {
        String::from_str(&_env, "Test Token")
    }

    pub fn symbol(_env: Env) -> String {
        String::from_str(&_env, "TT")
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().persistent().get(&TOTAL_SUPPLY).unwrap_or(0)
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().persistent().get(&(BALANCE, id)).unwrap_or(0)
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let key = AllowanceDataKey { from, spender };
        env.storage()
            .persistent()
            .get(&(ALLOWANCE, key))
            .unwrap_or(0)
    }

    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        from.require_auth();

        let key = AllowanceDataKey {
            from: from.clone(),
            spender: spender.clone(),
        };

        env.storage().persistent().set(&(ALLOWANCE, key), &amount);

        if expiration_ledger < env.ledger().sequence() {
            env.storage().persistent().extend_ttl(
                &(ALLOWANCE, AllowanceDataKey { from, spender }),
                100,
                100,
            );
        }
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        Self::transfer_impl(env, from, to, amount);
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        let allowance_key = AllowanceDataKey {
            from: from.clone(),
            spender: spender.clone(),
        };

        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowance < amount {
            panic!("insufficient allowance");
        }

        env.storage()
            .persistent()
            .set(&(ALLOWANCE, allowance_key), &(allowance - amount));
        Self::transfer_impl(env, from, to, amount);
    }

    pub fn mint(env: Env, admin: Address, to: Address, amount: i128) {
        admin.require_auth();

        let balance = Self::balance(env.clone(), to.clone());
        let total_supply = Self::total_supply(env.clone());

        env.storage()
            .persistent()
            .set(&(BALANCE, to), &(balance + amount));
        env.storage()
            .persistent()
            .set(&TOTAL_SUPPLY, &(total_supply + amount));
    }

    fn transfer_impl(env: Env, from: Address, to: Address, amount: i128) {
        let from_balance = Self::balance(env.clone(), from.clone());
        let to_balance = Self::balance(env.clone(), to.clone());

        if from_balance < amount {
            panic!("insufficient balance");
        }

        env.storage()
            .persistent()
            .set(&(BALANCE, from), &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&(BALANCE, to), &(to_balance + amount));
    }
}
