#![no_std]

use crate::maker_traits::MakerTraitsLib;
use crate::taker_traits::TakerTraitsLib;
use dutch_auction_interface::{AuctionDetails, DutchAuctionCalculatorContractClient};
use order_interface::Order;
use soroban_sdk::{
    contract, contractimpl, symbol_short, token::TokenClient, Address, Bytes, BytesN, Env, Symbol,
    U256,
};
use utils::math::min_num;
pub mod consts_trait;
pub mod maker_traits;
pub mod taker_traits;

const DUTCH_AUCTION_CALCULATOR_ADDRESS_KEY: Symbol = symbol_short!("DA_ADDY");
/// Order filled event
const ORDER_FILLED_EVENT_KEY: Symbol = symbol_short!("ORDR_F");
/**
 * OrderFilled(
 *   bytes32 orderHash,
 *   uint256 remainingMakingAmount
 * )
 */

#[contract]
pub struct OrderProtocol;

#[contractimpl]
impl OrderProtocol {
    pub fn __constructor(env: Env, da_addy: Address) {
        env.storage()
            .instance()
            .set(&DUTCH_AUCTION_CALCULATOR_ADDRESS_KEY, &da_addy);
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
        let da_addy = env
            .storage()
            .instance()
            .get(&DUTCH_AUCTION_CALCULATOR_ADDRESS_KEY)
            .unwrap();
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
        let da_addy = env
            .storage()
            .instance()
            .get(&DUTCH_AUCTION_CALCULATOR_ADDRESS_KEY)
            .unwrap();
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
        order_hash: BytesN<32>,
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
        let is_making_amount = TakerTraitsLib::is_making_amount(&env, &_taker_traits); // takerTraits.isMakingAmount
        if is_making_amount {
            let making_amount = min_num(&order.making_amount, &remaining_making_amount);

            let _taking_amount = Self::calculate_taking_amount(
                env.clone(),
                order.clone(),
                _extension,
                making_amount.clone(),
                remaining_making_amount.clone(),
                order_hash,
                auction_details,
            );

            let threshold: U256 = TakerTraitsLib::threshold(&env, _taker_traits.clone());

            if threshold > U256::from_u32(&env, 0) {
                if _amount == *making_amount {
                    if _taking_amount > threshold {
                        panic!("Taking amount too high");
                    }
                    if _taking_amount.mul(&_amount) > threshold.mul(&making_amount) {
                        panic!("Taking amount too high");
                    }
                }
            }
        } else {
            let taking_amount = _amount.clone();
            let making_amount = Self::calculate_making_amount(
                env.clone(),
                order.clone(),
                _extension.clone(),
                taking_amount.clone(),
                remaining_making_amount.clone(),
                order_hash.clone(),
                auction_details.clone(),
            );

            if making_amount > remaining_making_amount {
                let making_amount = remaining_making_amount.clone();
                let taking_amount = Self::calculate_taking_amount(
                    env.clone(),
                    order.clone(),
                    _extension.clone(),
                    making_amount.clone(),
                    remaining_making_amount.clone(),
                    order_hash.clone(),
                    auction_details.clone(),
                );

                if taking_amount > _amount {
                    panic!("Taking amount exceeded");
                }
            }

            let threshold: U256 = TakerTraitsLib::threshold(&env, _taker_traits.clone());

            if threshold > U256::from_u32(&env, 0) {
                if _amount == taking_amount {
                    if making_amount < threshold {
                        panic!("Making amount too low");
                    }
                    if making_amount.mul(&_amount) < threshold.mul(&taking_amount) {
                        panic!("Making amount too low");
                    }
                }
            }

            if !MakerTraitsLib::allow_partial_fills(&env, order.maker_traits.clone())
                && making_amount != order.making_amount
            {
                panic!("Partial fill not allowed")
            }

            if making_amount.mul(&taking_amount) == U256::from_u32(&env, 0) {
                panic!("Swap with zero amount");
            }

            // Invalidate order depending on makerTraits
            /*
               // Invalidate order depending on makerTraits
               if (order.makerTraits.useBitInvalidator()) {
                   _bitInvalidator[order.maker.get()].checkAndInvalidate(order.makerTraits.nonceOrEpoch());
               } else {
                   _remainingInvalidator[order.maker.get()][orderHash] = RemainingInvalidatorLib.remains(remainingMakingAmount, makingAmount);
               }
            */

            // one of this would be used. It's not right to use both.

            // Maker => Taker

            let making_amount_i128 = U256::to_u128(&making_amount).unwrap() as i128;

            TokenClient::new(&env, &order.maker_asset).transfer(
                &order.maker,
                &order.receiver,
                &making_amount_i128, // here we need to convert U256 to i128
            );

            // Taker => Maker
            let taking_amount_i128 = U256::to_u128(&taking_amount).unwrap() as i128;
            TokenClient::new(&env, &order.taker_asset).transfer(
                &order.receiver,
                &order.maker,
                &taking_amount_i128,
            );

            let amount = remaining_making_amount.sub(&making_amount);

            env.events()
                .publish((&ORDER_FILLED_EVENT_KEY, &order_hash, &amount), ());
        }
    }
}

mod maker_traits_test;
mod test;
