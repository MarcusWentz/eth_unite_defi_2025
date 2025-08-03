#![no_std]
use soroban_sdk::{contract, contractimpl, log, Address, Bytes, BytesN, Env, U256};

use dutch_auction_interface::DutchAuctionCalculatorInterface;
use order_interface::{AuctionDetails, Order};
use utils::math::{bitand, max_num, min_num};

const _LOW_128_BITS: u128 = 0xffffffffffffffffffffffffffffffff;
#[contract]
pub struct DutchAuctionCalculatorContract;

#[contractimpl]
impl DutchAuctionCalculatorInterface for DutchAuctionCalculatorContract {
    fn get_making_amount(
        env: Env,
        order: Order,
        _extension: Bytes,
        _order_hash: BytesN<32>,
        _taker: Address,
        taking_amount: U256,
        _remaining_making_amount: U256,
        auction_details: AuctionDetails,
    ) -> U256 {
        let calculated_taking_amount = Self::calculate_auction_taking_amount(
            env.clone(),
            auction_details.auction_start_time,
            auction_details.taking_amount_start,
            auction_details.taking_amount_end,
        );
        log!(
            &env,
            "calculated_taking_amount: {}",
            calculated_taking_amount
        );
        return order
            .making_amount
            .mul(&taking_amount)
            .div(&calculated_taking_amount);
    }

    fn get_taking_amount(
        env: Env,
        order: Order,
        _extension: Bytes,
        _order_hash: BytesN<32>,
        _taker: Address,
        making_amount: U256,
        _remaining_making_amount: U256,
        auction_details: AuctionDetails,
    ) -> U256 {
        let calculated_taking_amount = Self::calculate_auction_taking_amount(
            env.clone(),
            auction_details.auction_start_time,
            auction_details.taking_amount_start,
            auction_details.taking_amount_end,
        );

        //
        let numerator = calculated_taking_amount.mul(&making_amount);

        // divide and round up
        let denominator = order.making_amount;
        let adjustment = denominator.sub(&U256::from_u32(&env, 1));
        return numerator.add(&adjustment).div(&denominator);
    }

    fn calculate_auction_taking_amount(
        env: Env,
        auction_start_time: U256,
        taking_amount_start: U256,
        taking_amount_end: U256,
    ) -> U256 {
        // auction_start_time packs both start and end time into a single U256
        // The first 128 bits contain the start time, shifted right to extract it
        // let start_time = auction_start_time >> 128;
        let start_time = auction_start_time.shr(128);

        // The last 128 bits contain the end time, masked with _LOW_128_BITS to extract it
        let end_time = bitand(
            &env,
            auction_start_time,
            U256::from_u128(&env, _LOW_128_BITS),
        );

        // Get current time bounded between start and end time
        let block_time = U256::from_u128(&env, env.ledger().timestamp() as u128);
        let current_time = max_num(&start_time, min_num(&end_time, &block_time));

        (taking_amount_start
            .mul(&(end_time.sub(&current_time)))
            .add(&taking_amount_end.mul(&(current_time.sub(&start_time)))))
        .div(&(end_time.sub(&start_time)))
    }
}

mod test;
