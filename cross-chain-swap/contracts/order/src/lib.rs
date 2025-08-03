#![no_std]

use crate::{maker_traits::MakerTraitsLib, xlm_orders::{hash, domain_separator_v4}};
use crate::taker_traits::TakerTraitsLib;
use dutch_auction_interface::DutchAuctionCalculatorContractClient;
use order_interface::{AuctionDetails, Order, OrderInterface};
use soroban_sdk::{
    contract, contractimpl, crypto::{Hash}, symbol_short, token::TokenClient, xdr::{FromXdr, ToXdr}, Address, Bytes, BytesN, Env, Symbol, U256
};
use utils::math::min_num;
pub mod consts_trait;
pub mod maker_traits;
pub mod taker_traits;
pub mod xlm_orders;

const DUTCH_AUCTION_CALCULATOR_ADDRESS_KEY: Symbol = symbol_short!("DA_ADDY");
/// Order filled event
const ORDER_FILLED_EVENT_KEY: Symbol = symbol_short!("ORDR_F");
/**
 * OrderFilled(
 *   bytes32 orderHash,
 *   uint256 remainingMakingAmount
 * )
 */

fn check_signature(env: &Env, order: Order, r: BytesN<32>, vs: BytesN<32>) -> bool {
    let order_hash = hash(env, &order, &domain_separator_v4(env));
    let maker = order.maker;

    // types are tricky. The only way to get a Hash<32> is to use sha-3 first
    let _hash = Hash::from(env.crypto().keccak256(&order_hash.try_into().unwrap()));

    let mut signature = Bytes::new(env);
    signature.extend_from_array(&r.to_array());
    signature.extend_from_array(&vs.to_array());

    let recovered_maker = env.crypto().secp256k1_recover(
        &_hash,
        &signature.try_into().unwrap(),
        0,
    );
    let address_bytes = Bytes::from_xdr(&env, &maker.to_xdr(&env));
    if address_bytes.is_err() {
        panic!("Failed to convert address to bytes");
    }

    let recovered_maker_address = Bytes::from_xdr(&env, &recovered_maker.to_xdr(&env));
    if recovered_maker_address.is_err() {
        panic!("Failed to convert recovered maker to address");
    }

    return address_bytes.unwrap() == recovered_maker_address.unwrap();
}

/**
 * Parses the taker traits and args to get the target, extension, and interaction.
 * @param taker_traits The taker traits.
 * @param args The args.
 * @return target The target address.
 * @return extension The extension.
 * @return interaction The interaction calldata.
 * 
 * @return The target, extension, and interaction.
 */
fn parse_args(env: Env, taker_traits: U256, args: Bytes) -> (Address, Bytes, Bytes) {
    let mut target: Address;

    let mut args = args.clone();

    if TakerTraitsLib::args_has_target(&env, taker_traits.clone()) {
        let targetXdr = Address::from_xdr(&env, &args.clone().to_xdr(&env));
        if targetXdr.is_err() {
            panic!("Failed to convert args to address");
        }
        target = targetXdr.unwrap();
        args = args.slice(20..);

    } else {
        target = env.current_contract_address();
    }

    let extension_length = TakerTraitsLib::args_extension_length(&env, taker_traits.clone());
    let extension: Bytes = if extension_length > U256::from_u32(&env, 0) {
        let len = extension_length.to_u128().unwrap() as u32;
        let extension = args.slice(..len);
        args = args.slice(len..);
        extension
    } else {
        Bytes::new(&env)
    };

    let interaction_length = TakerTraitsLib::args_interaction_length(&env, taker_traits.clone());

    let interaction: Bytes = if interaction_length > U256::from_u32(&env, 0) {
        let len = interaction_length.to_u128().unwrap() as u32;
        let interaction = args.slice(..len);
        args = args.slice(len..);
        interaction
    } else {
        Bytes::new(&env)
    };

    (target, extension, interaction)
}

#[contract]
pub struct OrderProtocol;

#[contractimpl]
impl OrderInterface for OrderProtocol {
    fn __constructor(env: Env, da_addy: Address) {
        env.storage()
            .instance()
            .set(&DUTCH_AUCTION_CALCULATOR_ADDRESS_KEY, &da_addy);
    }

    fn calculate_making_amount(
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

    fn calculate_taking_amount(
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

    fn fill(
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
        let is_making_amount = TakerTraitsLib::is_making_amount(&env, &_taker_traits.clone()); // takerTraits.isMakingAmount
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

    fn _check_remaining_making_amount(env: Env, order: Order, order_hash: BytesN<32>) -> U256 {
        let mut remaining_making_amount = order.making_amount.clone();
        if MakerTraitsLib::use_bit_invalidator(&env, order.maker_traits.clone()) {
            remaining_making_amount = order.making_amount.clone();
        } else {
            // todo: implement this
            panic!("Not implemented");
        }
        return remaining_making_amount;
    }

    fn order_hash(env: Env, order: Order) -> BytesN<32> {
        hash(&env, &order, &domain_separator_v4(&env))
    }

    fn fill_order(
        env: Env,
        order: Order,
        r: BytesN<32>,
        vs: BytesN<32>,
        amount: U256,
        taker_traits: U256,
        target: Address,
        extension: Bytes,
        interaction: Bytes,
        auction_details: AuctionDetails,
    ) -> (U256, U256, BytesN<32>) {
        let order_hash = hash(&env, &order, &domain_separator_v4(&env));

        let remaining_making_amount = Self::_check_remaining_making_amount(env.clone(), order.clone(), order_hash.clone());

        if remaining_making_amount == order.making_amount {
            // let order_hash = hash(&env, &order.clone(), &domain_separator_v4(&env));

            // Checking signature
            let is_signature_valid = check_signature(&env, order.clone(), r.clone(), vs.clone());
            if !is_signature_valid {
                panic!("Invalid signature");
            }
        }

        Self::fill(env.clone(), order.clone(), order_hash.clone(), remaining_making_amount.clone(), amount.clone(), taker_traits.clone(), target.clone(), extension.clone(), interaction.clone(), auction_details.clone());

        return (remaining_making_amount, amount, order_hash);
    }

    fn fill_order_args(
        env: Env,
        order: Order,
        r: BytesN<32>,
        vs: BytesN<32>,
        taker_traits: U256,
        amount: U256,
        args: Bytes,
        auction_details: AuctionDetails,
    ) -> (U256, U256, BytesN<32>) {
        let (target, extension, interaction) = parse_args(env.clone(), taker_traits.clone(), args);
        return Self::fill_order(env, order, r, vs, amount, taker_traits, target, extension, interaction, auction_details);
    }
}

mod maker_traits_test;
mod taker_traits_test;
mod test;
mod xlm_orders_test;
