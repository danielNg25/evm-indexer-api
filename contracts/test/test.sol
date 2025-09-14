// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/src/Test.sol";
import {Constant} from "./Constant.t.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {UniversalRouter, CallbackType} from "../contracts/UniversalRouter.sol";
import {UniversalRouterCustomFee} from "../contracts/UniversalRouterCustomFee.sol";

contract ForkUniversalDexLoopingHookTest is Constant, Test {
    UniversalRouterCustomFee public universalRouter;

    function setUp() public {
        vm.createSelectFork("https://rpc.ankr.com/flare", 46659101);
    }

    function testMulticall() public {
        UniversalRouterCustomFee(
            payable(0x7fb0FC76b35cc16C239A7342d84cB77Ce926b151)
        ).swap(
                7040443286352,
                hex"60fdc7b744e886e96aa0def5f69ee440db9d8c77ce629b3ea0ccaba57581be1f24553e0471e1b9b7140d8d3649ec605cf69018c627fb44ccc76ec89fbdd80c82000151f355e523fed0b5fa32a07d12a1ff56eb5b1a7faa972291117e5e9565da29bc808de388298dfcc0da8f81582e2c4f7df92b46703a83"
            );
    }
}

