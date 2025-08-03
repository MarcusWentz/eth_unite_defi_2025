// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "../src/EscrowFactory.sol";

contract DeployEscrowFactory is Script {
    function run() external {
        // Use default Anvil private key for local deployment
        uint256 deployerPrivateKey = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;
        
        vm.startBroadcast(deployerPrivateKey);
        
        // Deploy EscrowFactory
        EscrowFactory escrowFactory = new EscrowFactory();
        
        console.log("EscrowFactory deployed at:", address(escrowFactory));
        
        vm.stopBroadcast();
    }
} 