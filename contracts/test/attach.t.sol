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
        vm.createSelectFork("https://rpc.ankr.com/etherlink_mainnet", 24277076); // Fork from mainnet

        // universalRouter = UniversalRouterCustomFee(
        //     payable(0x211820c9CF8eC1884766613cBa276f4416106b25)
        // );
        universalRouter = UniversalRouterCustomFee(
            payable(0x07391EC5F12A3A8839fDA4dA5D7CdE42Aa0C5A8a)
        );
    }

    function testLoopp() public {
        vm.startPrank(0xF90E54B26866edBffedAaAC63587163c0305fff7);

        uint256 amountOut = 21.371804632897426831 ether;
        console2.log("amountOut", amountOut);

        universalRouter.swap(
            amountOut,
            abi.encodePacked(
                address(0xbFc94CD2B1E55999Cfc7347a9313e88702B83d0F),
                address(0x2a5120e8B04E7F2D3fbbbD82afb4CD70de0F5d0e),
                address(0x9121B153bbCF8C23F20eE43b494F08760B91aD64),
                address(0x54c91605D2dfE7f0C98cc3E517262B0ac8b4A6c5),
                address(0xc9B53AB2679f573e480d01e0f49e2B5CFB7a3EAb),
                address(0x93e808119Ff3D772160ad40f2C54b319E8A82f6c)
            )
        );

        console2.log(
            IERC20(0xc9B53AB2679f573e480d01e0f49e2B5CFB7a3EAb).balanceOf(
                0xF90E54B26866edBffedAaAC63587163c0305fff7
            )
        );
        // address(universalRouter).call(
        //     hex"bd0625ab000000000000000000000000000000000000000000000000008db5f29a3a15020000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000007815140000000000000000000000000000000000007e399b4a2414d90f17fbf1327e80da57624b041bf1815bd50389c46847f0bda824ec8da914045d140012dc8daed76fd687b51ad6a4683aa151d3d59dbab93b7ad7fe8692a878b95a8e689423437cc5001bd56a4eb84e3ea57efa35526c15f8b50c17f41d0000000000000000"
        // );
    }
}
// [2025-08-28 05:53:39] FAILED Block: 24277076 | Profit: 0.142789941124438792 | Tx: 0x72c16dd8c768d7120b2a3714628028e3a8ae8e265f316b89b721105da956b770 | Source: 0xfef8b438c2b785a23a659d58c04b54d6eadfcb7e567861b9f632e39fd5f7fdcc | Amount: 21.371804632897426831 |
// Path: 0xc9B53AB2679f573e480d01e0f49e2B5CFB7a3EAb->0x93e808119Ff3D772160ad40f2C54b319E8A82f6c
// --> 0x9121B153bbCF8C23F20eE43b494F08760B91aD64->0x54c91605D2dfE7f0C98cc3E517262B0ac8b4A6c5
// --> 0xbFc94CD2B1E55999Cfc7347a9313e88702B83d0F->0x2a5120e8B04E7F2D3fbbbD82afb4CD70de0F5d0e
// Metrics: TX: 0xfef8b438c2b785a23a659d58c04b54d6eadfcb7e567861b9f632e39fd5f7fdcc | Received at: 2025-08-28 05:53:32.270 | Proccessed at: 2025-08-28 05:53:32.270 (+0ms) | Simulated at: 2025-08-28 05:53:32.272 (+2ms) | Sent at: 2025-08-28 05:53:32.272 (+0ms) | Executed at: 2025-08-28 05:53:39.654 (+7382ms) | Total time: 2ms
//  | Steps: 21371804632897426831, 7030225, 15188, 21514594574021865623
