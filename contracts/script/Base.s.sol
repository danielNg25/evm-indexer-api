// SPDX-License-Identifier: MIT
pragma solidity >=0.8.23 <0.9.0;

import {Script} from "forge-std/src/Script.sol";

abstract contract BaseScript is Script {
    /// @dev The address of the transaction broadcaster.
    address internal broadcaster;

    /// @dev Used to derive the broadcaster's address if $ETH_FROM is not defined.
    uint256 internal privateKey;

    /// @dev Initializes the transaction broadcaster like this:
    ///
    /// - If $ETH_FROM is defined, use it.
    /// - Otherwise, derive the broadcaster address from $MNEMONIC.
    /// - If $MNEMONIC is not defined, default to a test mnemonic.
    ///
    /// The use case for $ETH_FROM is to specify the broadcaster key and its address via the command line.
    constructor() {
        privateKey = vm.envUint("PRIVATE_KEY");
        broadcaster = vm.addr(privateKey);
    }

    modifier broadcast() {
        vm.startBroadcast(privateKey);
        _;
        vm.stopBroadcast();
    }
}
