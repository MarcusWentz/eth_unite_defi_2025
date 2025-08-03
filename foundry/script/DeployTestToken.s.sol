// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestToken is ERC20 {
    constructor() ERC20("Test USDC", "tUSDC") {
        _mint(msg.sender, 1000000 * 10**decimals());
    }
}

contract DeployTestToken is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        vm.startBroadcast(deployerPrivateKey);
        
        // Deploy TestToken
        TestToken testToken = new TestToken();
        
        console.log("TestToken deployed at:", address(testToken));
        
        vm.stopBroadcast();
    }
} 