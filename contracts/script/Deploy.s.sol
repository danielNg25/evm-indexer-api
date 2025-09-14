// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.23 <0.9.0;

import {BaseScript} from "./Base.s.sol";
import {console2} from "forge-std/src/console2.sol";
import {IPool} from "@aave/core-v3/contracts/interfaces/IPool.sol";

import {UniversalRouter, CallbackType} from "../contracts/UniversalRouter.sol";
import {Constant} from "./Constant.s.sol";

/// @dev See the Solidity Scripting tutorial: https://book.getfoundry.sh/tutorials/solidity-scripting
contract Deploy is BaseScript, Constant {
    function run() public broadcast {
        console2.log("Deployer", broadcaster);
        bytes4[] memory sigs = new bytes4[](7);
        sigs[0] = bytes4(
            keccak256("uniswapV3SwapCallback(int256,int256,bytes)")
        );
        sigs[1] = bytes4(
            keccak256("pancakeV3SwapCallback(int256,int256,bytes)")
        );
        sigs[2] = bytes4(
            keccak256("uniswapV2Call(address,uint256,uint256,bytes)")
        );
        sigs[3] = bytes4(
            keccak256("pancakeCall(address,uint256,uint256,bytes)")
        );
        sigs[4] = bytes4(
            keccak256("storyHuntV3SwapCallback(int256,int256,bytes)")
        );
        sigs[5] = bytes4(
            keccak256("PiperXswapV2Call(address,uint256,uint256,bytes)")
        );
        sigs[6] = bytes4(
            keccak256("swapV2Call(address,uint256,uint256,bytes)")
        );

        CallbackType[] memory callbackTypes = new CallbackType[](7);
        callbackTypes[0] = CallbackType.SwapV3;
        callbackTypes[1] = CallbackType.SwapV3;
        callbackTypes[2] = CallbackType.SwapV2;
        callbackTypes[3] = CallbackType.SwapV2;
        callbackTypes[4] = CallbackType.SwapV3;
        callbackTypes[5] = CallbackType.SwapV2;
        callbackTypes[6] = CallbackType.SwapV2;

        UniversalRouter universalRouter = new UniversalRouter(
            sigs,
            callbackTypes
        );
        console2.log("UniversalRouter deployed at", address(universalRouter));
    }
}
