/**
// SPDX-License-Identifier: MIT

pragma solidity 0.8.23;

import "@openzeppelin/contracts/utils/math/Math.sol";
import "../interfaces/IAmountGetter.sol";

/// @title A helper that implements price decay over time from max to min
/// @notice The contract implements Dutch auction price calculation for 1inch limit orders, it is used by 1inch Fusion
contract DutchAuctionCalculator is IAmountGetter {
    using Math for uint256;

    uint256 private constant _LOW_128_BITS = 0xffffffffffffffffffffffffffffffff;

    function getMakingAmount(
        IOrderMixin.Order calldata order,
        bytes calldata /* extension */,
        bytes32 /* orderHash */,
        address /* taker */,
        uint256 takingAmount,
        uint256 /* remainingMakingAmount */,
        bytes calldata extraData
    ) external view returns (uint256) {
        (
            uint256 startTimeEndTime,
            uint256 takingAmountStart,
            uint256 takingAmountEnd
        ) = abi.decode(extraData, (uint256, uint256, uint256));

        uint256 calculatedTakingAmount = _calculateAuctionTakingAmount(startTimeEndTime, takingAmountStart, takingAmountEnd);
        return order.makingAmount * takingAmount / calculatedTakingAmount;
    }

    function getTakingAmount(
        IOrderMixin.Order calldata order,
        bytes calldata /* extension */,
        bytes32 /* orderHash */,
        address /* taker */,
        uint256 makingAmount,
        uint256 /* remainingMakingAmount */,
        bytes calldata extraData
    ) external view returns (uint256) {
        (
            uint256 startTimeEndTime,
            uint256 takingAmountStart,
            uint256 takingAmountEnd
        ) = abi.decode(extraData, (uint256, uint256, uint256));

        uint256 calculatedTakingAmount = _calculateAuctionTakingAmount(startTimeEndTime, takingAmountStart, takingAmountEnd);
        return (calculatedTakingAmount * makingAmount).ceilDiv(order.makingAmount);
    }

    function _calculateAuctionTakingAmount(uint256 startTimeEndTime, uint256 takingAmountStart, uint256 takingAmountEnd) private view returns(uint256) {
        uint256 startTime = startTimeEndTime >> 128;
        uint256 endTime = startTimeEndTime & _LOW_128_BITS;
        uint256 currentTime = Math.max(startTime, Math.min(endTime, block.timestamp));  // solhint-disable-line not-rely-on-time
        return (takingAmountStart * (endTime - currentTime) + takingAmountEnd * (currentTime - startTime)) / (endTime - startTime);
    }
}

**/
use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, BytesN, Env, U256};

use crate::Order;

const _LOW_128_BITS: u128 = 0xffffffffffffffffffffffffffffffff;

fn max_num(a: U256, b: U256) -> U256 {
    if a >= b {
        a
    } else {
        b
    }
}

fn min_num(a: U256, b: U256) -> U256 {
    if a < b {
        a
    } else {
        b
    }
}

#[contracttype]
pub struct AuctionDetails {
    auction_start_time: U256,
    taking_amount_start: U256,
    taking_amount_end: U256,
}

#[contract]
pub struct DutchAuctionCalculator {}

#[contractimpl]
impl DutchAuctionCalculator {
    pub fn get_making_amount(
        env: &Env,
        order: Order,
        _extension: Bytes,
        _order_hash: BytesN<32>,
        _taker: Address,
        taking_amount: U256,
        _remaining_making_amount: U256,
        auction_details: AuctionDetails,
    ) -> U256 {
        let calculated_taking_amount = Self::calculate_auction_taking_amount(
            env,
            auction_details.auction_start_time,
            auction_details.taking_amount_start,
            auction_details.taking_amount_end,
        );
        return order.making_amount * taking_amount / calculated_taking_amount;
    }

    fn calculate_auction_taking_amount(
        env: &Env,
        auction_start_time: U256,
        taking_amount_start: U256,
        taking_amount_end: U256,
    ) -> U256 {
        // auction_start_time packs both start and end time into a single U256
        // The first 128 bits contain the start time, shifted right to extract it
        // let start_time = auction_start_time >> 128;
        let start_time = auction_start_time.to_be_bytes() >> 128;
        
        // The last 128 bits contain the end time, masked with _LOW_128_BITS to extract it
        let end_time = auction_start_time & _LOW_128_BITS;
        
        // Get current time bounded between start and end time
        let current_time = max_num(start_time, min_num(end_time, env.ledger().timestamp()));
        (taking_amount_start * (end_time - current_time)
            + taking_amount_end * (current_time - start_time))
            / (end_time - start_time)
    }
}
