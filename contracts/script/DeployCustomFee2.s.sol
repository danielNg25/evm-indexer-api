// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.23 <0.9.0;

import {BaseScript} from "./Base.s.sol";
import {console2} from "forge-std/src/console2.sol";
import {IPool} from "@aave/core-v3/contracts/interfaces/IPool.sol";

import {UniversalRouterCustomFee, CallbackType} from "../contracts/UniversalRouterCustomFee.sol";
import {Constant} from "./Constant.s.sol";

/// @dev See the Solidity Scripting tutorial: https://book.getfoundry.sh/tutorials/solidity-scripting
contract Deploy is BaseScript, Constant {
    function run() public broadcast {
        console2.log("Deployer", broadcaster);
        bytes4[] memory sigs = new bytes4[](6);
        sigs[0] = bytes4(keccak256("algebraSwapCallback(int256,int256,bytes)"));
        sigs[1] = bytes4(
            keccak256("pancakeV3SwapCallback(int256,int256,bytes)")
        );
        sigs[2] = bytes4(
            keccak256("uniswapV2Call(address,uint256,uint256,bytes)")
        );
        sigs[3] = bytes4(
            keccak256("somnexAMMCall(address,uint256,uint256,bytes)")
        );
        sigs[4] = bytes4(
            keccak256("uniswapV3SwapCallback(int256,int256,bytes)")
        );
        sigs[5] = bytes4(
            keccak256("somniaExchangeCall(address,uint256,uint256,bytes)")
        );
        console2.logBytes4(sigs[0]);
        console2.logBytes4(sigs[1]);
        console2.logBytes4(sigs[2]);
        console2.logBytes4(sigs[3]);
        console2.logBytes4(sigs[4]);
        console2.logBytes4(sigs[5]);

        // Define callback types
        CallbackType[] memory callbackTypes = new CallbackType[](6);
        callbackTypes[0] = CallbackType.SwapV3; // uniswapV3SwapCallback
        callbackTypes[1] = CallbackType.SwapV3; // elkDexV3SwapCallback
        callbackTypes[2] = CallbackType.SwapV2; // mimoCall
        callbackTypes[3] = CallbackType.SwapV2; // hook
        callbackTypes[4] = CallbackType.SwapV3; // dragonswapV2SwapCallback
        callbackTypes[5] = CallbackType.SwapV2; // algebraSwapCallback

        // Define V2 factory addresses
        address[] memory v2Factories = new address[](3);
        v2Factories[0] = address(1); // default - hook
        v2Factories[1] = 0x6C4853C97b981Aa848C2b56F160a73a46b5DCCD4; // mimoCall
        v2Factories[2] = 0xaFd71143Fb155058e96527B07695D93223747ed1; // mimoCall

        // Define V2 fees
        uint256[] memory v2Fees = new uint256[](3);
        v2Fees[0] = 50; // elkCall
        v2Fees[1] = 30; // default
        v2Fees[2] = 50; // default

        UniversalRouterCustomFee universalRouter = new UniversalRouterCustomFee(
            sigs,
            callbackTypes,
            v2Factories,
            v2Fees
        );

        console2.log("UniversalRouter deployed at", address(universalRouter));
    }
}
