// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "../src/EscrowFactory.sol";

contract DeployEscrowFactory is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        vm.startBroadcast(deployerPrivateKey);
        
        // Deploy EscrowFactory
        EscrowFactory escrowFactory = new EscrowFactory();
        
        console.log("EscrowFactory deployed at:", address(escrowFactory));
        
        vm.stopBroadcast();
    }
} 