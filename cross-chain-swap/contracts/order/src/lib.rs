#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, BytesN, Env, U256, symbol_short, Symbol};
use crate::maker_traits::MakerTraitsLib;
use dutch_auction_interface::{DutchAuctionCalculatorContractClient, AuctionDetails};
use order_interface::Order;
pub mod consts_trait;
pub mod maker_traits;
pub mod taker_traits;

const DUTCH_AUCTION_CALCULATOR_ADDRESS: Symbol = symbol_short!("DA_ADDY");

#[contract]
pub struct OrderProtocol;


#[contractimpl]
impl OrderProtocol {
    pub fn __constructor(env: Env, da_addy: Address) {
        env.storage().instance().set(&DUTCH_AUCTION_CALCULATOR_ADDRESS, &da_addy);
    }

    pub fn calculate_making_amount(
        env: Env,
        order: Order,
        _extension: Bytes,
        requested_taking_amount: U256,
        remaining_making_amount: U256,
        order_hash: BytesN<32>,
        auction_details: AuctionDetails,
    ) -> U256 {
        let da_addy = env.storage().instance().get(&DUTCH_AUCTION_CALCULATOR_ADDRESS).unwrap();
        let da_client = DutchAuctionCalculatorContractClient::new(&env, &da_addy);

        let da_result = da_client.get_making_amount(
            &order,
            &_extension,
            &order_hash,
            &order.receiver,
            &requested_taking_amount,
            &remaining_making_amount,
            &auction_details,
        );

        return da_result;
    }

    pub fn calculate_taking_amount(
        env: Env,
        order: Order,
        _extension: Bytes,
        requested_making_amount: U256,
        remaining_making_amount: U256,
        order_hash: BytesN<32>,
        auction_details: AuctionDetails,
    ) -> U256 {
        let da_addy = env.storage().instance().get(&DUTCH_AUCTION_CALCULATOR_ADDRESS).unwrap();
        let da_client = DutchAuctionCalculatorContractClient::new(&env, &da_addy);

        let da_result = da_client.get_taking_amount(
            &order,
            &_extension,
            &order_hash,
            &order.receiver,
            &requested_making_amount,
            &remaining_making_amount,
            &auction_details,
        );

        return da_result;
    }

    pub fn fill(
        env: Env,
        order: Order,
        _order_hash: BytesN<32>,
        remaining_making_amount: U256,
        _amount: U256,
        _taker_traits: U256,
        _target: Address,
        _extension: Bytes,
        _interaction: Bytes,
        auction_details: AuctionDetails,
    ) {
        // ignoring _extension validation phase.

        if !MakerTraitsLib::is_allowed_sender(&env, order.maker_traits.clone(), _target) {
            panic!("Private order");
        }

        if MakerTraitsLib::is_expired(&env, order.maker_traits.clone()) {
            panic!("Order expired");
        }

        if MakerTraitsLib::need_check_epoch_manager(&env, order.maker_traits.clone()) {
            if MakerTraitsLib::use_bit_invalidator(&env, order.maker_traits.clone()) {
                panic!("Epoch manager and bit invalidators are incompatible");
            }
            // todo: @Skanislav implement check:
            // if (!epochEquals(order.maker.get(), order.makerTraits.series(), order.makerTraits.nonceOrEpoch())) revert WrongSeriesNonce();
        }

        // ignoring extension predicate check.

        // Checks if the taking amount should be calculated based on making amount.
        let is_making_amount = false; // takerTraits.isMakingAmount
        if is_making_amount {
            let _taking_amount = Self::calculate_taking_amount(
                env,
                order,
                _extension,
                _amount,
                remaining_making_amount,
                _order_hash,
                auction_details,
            );
        } else {
            let _taking_amount = Self::calculate_making_amount(
                env,
                order,
                _extension,
                _amount,
                remaining_making_amount,
                _order_hash,
                auction_details,
            );
        }


    }
}

mod test;
