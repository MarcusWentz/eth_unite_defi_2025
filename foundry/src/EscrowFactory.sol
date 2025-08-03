// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";

contract EscrowFactory is Ownable {
    event EscrowCreated(address indexed escrow, address indexed maker, bytes32 indexed hashlock);
    
    mapping(address => bool) public authorizedResolvers;
    
    constructor() Ownable(msg.sender) {}
    
    function addResolver(address resolver) external onlyOwner {
        authorizedResolvers[resolver] = true;
    }
    
    function removeResolver(address resolver) external onlyOwner {
        authorizedResolvers[resolver] = false;
    }
    
    function createEscrow(
        address maker,
        address taker,
        address token,
        uint256 amount,
        bytes32 hashlock,
        uint256 timelock
    ) external returns (address) {
        require(authorizedResolvers[msg.sender], "Unauthorized resolver");
        
        // In a real implementation, this would deploy an actual escrow contract
        // For demo purposes, we'll just emit an event
        emit EscrowCreated(address(0), maker, hashlock);
        
        return address(0);
    }
} 