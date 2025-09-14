// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.23 <0.9.0;

import {BaseScript} from "./Base.s.sol";
import {console2} from "forge-std/src/console2.sol";
import {IPool} from "@aave/core-v3/contracts/interfaces/IPool.sol";

import {UniversalRouterCustomFee, CallbackType} from "../contracts/UniversalRouterCustomFee.sol";
import {Constant} from "./Constant.s.sol";

/// @dev See the Solidity Scripting tutorial: https://book.getfoundry.sh/tutorials/solidity-scripting
contract SetCalback is BaseScript, Constant {
    function run() public broadcast {
        console2.log("Deployer", broadcaster);
        UniversalRouterCustomFee universalRouter = UniversalRouterCustomFee(
            payable(address(0x3007BD82acDC9aB1d4EEDCB3e7a46Ec3Ac62A25a))
        );

        universalRouter.setV2FactoryFee(
            address(0x9c70B6B8e389b2C97090FFFA6bE3a13626ba3018),
            30
        );
        universalRouter.setV2FactoryFee(
            address(0x061715D0e7b91d436a4a57419d013dED490c264D),
            30
        );

        bytes4[] memory sigs = new bytes4[](2);
        sigs[0] = bytes4(0x84800812);
        sigs[1] = bytes4(0x1e71fa15);

        // Define callback types
        CallbackType[] memory callbackTypes = new CallbackType[](2);
        callbackTypes[0] = CallbackType.SwapV2; // uniswapV3SwapCallback
        callbackTypes[1] = CallbackType.SwapV2; // elkDexV3SwapCallback

        universalRouter.setApprovedSigs(sigs, callbackTypes);
    }
}
