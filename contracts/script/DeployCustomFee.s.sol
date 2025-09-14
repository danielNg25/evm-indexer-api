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
        // bytes4[] memory sigs = new bytes4[](21);
        // sigs[0] = bytes4(
        //     keccak256("uniswapV3SwapCallback(int256,int256,bytes)")
        // );
        // sigs[1] = bytes4(
        //     keccak256("pancakeV3SwapCallback(int256,int256,bytes)")
        // );
        // sigs[2] = bytes4(
        //     keccak256("uniswapV2Call(address,uint256,uint256,bytes)")
        // );
        // sigs[3] = bytes4(
        //     keccak256("pancakeCall(address,uint256,uint256,bytes)")
        // );
        // Flare
        // sigs[4] = bytes4(
        //     keccak256("blazeSwapCall(address,uint256,uint256,bytes)")
        // );
        // sigs[5] = bytes4(
        //     keccak256("enosysDexCall(address,uint256,uint256,bytes)")
        // );
        // sigs[6] = bytes4(
        //     keccak256("pangolinCall(address,uint256,uint256,bytes)")
        // );
        // sigs[7] = bytes4(
        //     keccak256("enosysdexV3SwapCallback(int256,int256,bytes)")
        // );

        // Soneium
        // sigs[4] = bytes4(keccak256("dyorCall(address,uint256,uint256,bytes)"));
        // sigs[5] = bytes4(
        //     keccak256("soneFiAMMCall(address,uint256,uint256,bytes)")
        // );
        // sigs[6] = bytes4(keccak256("hook(address,uint256,uint256,bytes)"));

        // Fraxtal
        // sigs[4] = bytes4(
        //     keccak256("ramsesV2SwapCallback(int256,int256,bytes)")
        // );
        // sigs[5] = bytes4(keccak256("hook(address,uint256,uint256,bytes)"));

        // Bera
        // sigs[4] = bytes4(keccak256("dyorCall(address,uint256,uint256,bytes)"));
        // sigs[5] = bytes4(
        //     keccak256("cheeseswapCall(address,uint256,uint256,bytes)")
        // );

        // Kava
        // sigs[4] = bytes4(
        //     keccak256("ramsesV2SwapCallback(int256,int256,bytes)")
        // );
        // sigs[5] = bytes4(
        //     keccak256("pangolinv3SwapCallback(int256,int256,bytes)")
        // );
        // sigs[6] = bytes4(
        //     keccak256("donaswapV3SwapCallback(int256,int256,bytes)")
        // );
        // sigs[7] = bytes4(keccak256("joeCall(address,uint256,uint256,bytes)"));
        // sigs[8] = bytes4(keccak256("arenaCall(address,uint256,uint256,bytes)"));
        // sigs[9] = bytes4(
        //     keccak256("pangolinCall(address,uint256,uint256,bytes)")
        // );
        // sigs[10] = bytes4(
        //     keccak256("sicleCall(address,uint256,uint256,bytes)")
        // );
        // sigs[11] = bytes4(
        //     keccak256("lydiaCall(address,uint256,uint256,bytes)")
        // );
        // sigs[12] = bytes4(
        //     keccak256("alligatorCall(address,uint256,uint256,bytes)")
        // );
        // sigs[13] = bytes4(
        //     keccak256("forwardCall(address,uint256,uint256,bytes)")
        // );
        // sigs[14] = bytes4(keccak256("elkCall(address,uint256,uint256,bytes)"));
        // sigs[15] = bytes4(
        //     keccak256("yetiswapCall(address,uint256,uint256,bytes)")
        // );
        // sigs[16] = bytes4(keccak256("hook(address,uint256,uint256,bytes)"));

        // core
        // sigs[4] = bytes4(keccak256("algebraSwapCallback(int256,int256,bytes)"));
        // sigs[5] = bytes4(
        //     keccak256("archerswapCall(address,uint256,uint256,bytes)")
        // );
        // sigs[6] = bytes4(
        //     keccak256("shadowCall(address,uint256,uint256,bytes)")
        // );
        // sigs[7] = bytes4(
        //     keccak256("freeswapCall(address,uint256,uint256,bytes)")
        // );
        // sigs[8] = bytes4(keccak256("swapCall(address,uint256,uint256,bytes)"));
        // sigs[9] = bytes4(keccak256("jwapCall(address,uint256,uint256,bytes)"));
        // sigs[10] = bytes4(keccak256("KywCall(address,uint256,uint256,bytes)"));
        // sigs[11] = bytes4(
        //     keccak256("migoswapCall(address,uint256,uint256,bytes)")
        // );
        // sigs[12] = bytes4(
        //     keccak256("crestswapCall(address,uint256,uint256,bytes)")
        // );
        // sigs[13] = bytes4(keccak256("wizCall(address,uint256,uint256,bytes)"));
        // sigs[14] = bytes4(keccak256("CuanCall(address,uint256,uint256,bytes)"));
        // sigs[15] = bytes4(
        //     keccak256("infinityCall(address,uint256,uint256,bytes)")
        // );
        // sigs[16] = bytes4(keccak256("woofCall(address,uint256,uint256,bytes)"));
        // sigs[17] = bytes4(
        //     keccak256("SwadiraCall(address,uint256,uint256,bytes)")
        // );
        // sigs[18] = bytes4(keccak256("BtccCall(address,uint256,uint256,bytes)"));
        // sigs[19] = bytes4(
        //     keccak256("CoreswapCall(address,uint256,uint256,bytes)")
        // );
        // sigs[20] = bytes4(
        //     keccak256("PalmaswapCall(address,uint256,uint256,bytes)")
        // );

        // avalanche
        bytes4[] memory sigs = new bytes4[](54);
        sigs[0] = bytes4(
            keccak256("ramsesV2SwapCallback(int256,int256,bytes)")
        );
        sigs[1] = bytes4(
            keccak256("uniswapV3SwapCallback(int256,int256,bytes)")
        );
        sigs[2] = bytes4(keccak256("joeCall(address,uint256,uint256,bytes)"));
        sigs[3] = bytes4(keccak256("arenaCall(address,uint256,uint256,bytes)"));
        sigs[4] = bytes4(
            keccak256("pangolinCall(address,uint256,uint256,bytes)")
        );
        sigs[5] = bytes4(
            keccak256("uniswapV2Call(address,uint256,uint256,bytes)")
        );
        sigs[6] = bytes4(
            keccak256("pangolinv3SwapCallback(int256,int256,bytes)")
        );
        sigs[7] = bytes4(keccak256("sicleCall(address,uint256,uint256,bytes)"));
        sigs[8] = bytes4(
            keccak256("donaswapV3SwapCallback(int256,int256,bytes)")
        );
        sigs[9] = bytes4(keccak256("lydiaCall(address,uint256,uint256,bytes)"));
        sigs[10] = bytes4(
            keccak256("alligatorCall(address,uint256,uint256,bytes)")
        );
        sigs[11] = bytes4(
            keccak256("forwardCall(address,uint256,uint256,bytes)")
        );
        sigs[12] = bytes4(keccak256("elkCall(address,uint256,uint256,bytes)"));
        sigs[13] = bytes4(
            keccak256("yetiswapCall(address,uint256,uint256,bytes)")
        );
        sigs[14] = bytes4(
            keccak256("hakuswapCall(address,uint256,uint256,bytes)")
        );
        sigs[15] = bytes4(
            keccak256("canaryCall(address,uint256,uint256,bytes)")
        );
        sigs[16] = bytes4(
            keccak256("complusV2Call(address,uint256,uint256,bytes)")
        );
        sigs[17] = bytes4(keccak256("swapCall(address,uint256,uint256,bytes)"));
        sigs[18] = bytes4(
            keccak256("VaporDEXCall(address,uint256,uint256,bytes)")
        );
        sigs[19] = bytes4(
            keccak256("soulswapCall(address,uint256,uint256,bytes)")
        );
        sigs[20] = bytes4(keccak256("swapCallback(int256,int256,bytes)"));
        sigs[21] = bytes4(
            keccak256("partyCall(address,uint256,uint256,bytes)")
        );
        sigs[22] = bytes4(
            keccak256("elkDexV3SwapCallback(int256,int256,bytes)")
        );
        sigs[23] = bytes4(
            keccak256("contextCall(address,uint256,uint256,bytes)")
        );
        sigs[24] = bytes4(
            keccak256("flashLiquidityCall(address,uint256,uint256,bytes)")
        );
        sigs[25] = bytes4(
            keccak256("pancakeCall(address,uint256,uint256,bytes)")
        );
        sigs[26] = bytes4(
            keccak256("StormCall(address,uint256,uint256,bytes)")
        );
        sigs[27] = bytes4(
            keccak256("AvaxPadSwapCall(address,uint256,uint256,bytes)")
        );
        sigs[28] = bytes4(keccak256("moeCall(address,uint256,uint256,bytes)"));
        sigs[29] = bytes4(
            keccak256("oliveCall(address,uint256,uint256,bytes)")
        );
        sigs[30] = bytes4(keccak256("elixirSwapCallback(int256,int256,bytes)"));
        sigs[31] = bytes4(
            keccak256("baguetteCall(address,uint256,uint256,bytes)")
        );
        sigs[32] = bytes4(
            keccak256("ruggyCall(address,uint256,uint256,bytes)")
        );
        sigs[33] = bytes4(keccak256("Call(address,uint256,uint256,bytes)"));
        sigs[34] = bytes4(keccak256("lpCall(address,uint256,uint256,bytes)"));
        sigs[35] = bytes4(0x4a639df4);
        sigs[36] = bytes4(keccak256("apexCall(address,uint256,uint256,bytes)"));
        sigs[37] = bytes4(
            keccak256("viralswapCall(address,uint256,uint256,bytes)")
        );
        sigs[38] = bytes4(
            keccak256("unifiCall(address,uint256,uint256,bytes)")
        );
        sigs[39] = bytes4(keccak256("ovxCall(address,uint256,uint256,bytes)"));
        sigs[40] = bytes4(
            keccak256("whaleswapCall(address,uint256,uint256,bytes)")
        );
        sigs[41] = bytes4(0xd66af394);
        sigs[42] = bytes4(
            keccak256("AzurSwapV2Call(address,uint256,uint256,bytes)")
        );
        sigs[43] = bytes4(keccak256("zeroCall(address,uint256,uint256,bytes)"));
        sigs[44] = bytes4(
            keccak256("CandyManCall(address,uint256,uint256,bytes)")
        );
        sigs[45] = bytes4(
            keccak256("otbSwapCall(address,uint256,uint256,bytes)")
        );
        sigs[46] = bytes4(
            keccak256("zswapCall(address,uint256,uint256,bytes)")
        );
        sigs[47] = bytes4(keccak256("hook(address,uint256,uint256,bytes)"));
        sigs[48] = bytes4(keccak256("hook(int256,int256,bytes)"));
        sigs[49] = bytes4(
            keccak256("uniswapV3Call(address,uint256,uint256,bytes)")
        );
        sigs[50] = bytes4(
            keccak256("swapV2Call(address,uint256,uint256,bytes)")
        );
        sigs[51] = bytes4(
            keccak256("swapV3Call(address,uint256,uint256,bytes)")
        );
        sigs[52] = bytes4(keccak256("v3SwapCallback(int256,int256,bytes)"));
        sigs[53] = bytes4(keccak256("v2Call(address,uint256,uint256,bytes)"));

        // Define callback types
        CallbackType[] memory callbackTypes = new CallbackType[](54);
        callbackTypes[0] = CallbackType.SwapV3; // ramsesV2SwapCallback
        callbackTypes[1] = CallbackType.SwapV3; // uniswapV3SwapCallback
        callbackTypes[2] = CallbackType.SwapV2; // joeCall
        callbackTypes[3] = CallbackType.SwapV2; // arenaCall
        callbackTypes[4] = CallbackType.SwapV2; // pangolinCall
        callbackTypes[5] = CallbackType.SwapV2; // uniswapV2Call
        callbackTypes[6] = CallbackType.SwapV3; // pangolinv3SwapCallback
        callbackTypes[7] = CallbackType.SwapV2; // sicleCall
        callbackTypes[8] = CallbackType.SwapV3; // donaswapV3SwapCallback
        callbackTypes[9] = CallbackType.SwapV2; // lydiaCall
        callbackTypes[10] = CallbackType.SwapV2; // alligatorCall
        callbackTypes[11] = CallbackType.SwapV2; // forwardCall
        callbackTypes[12] = CallbackType.SwapV2; // elkCall
        callbackTypes[13] = CallbackType.SwapV2; // yetiswapCall
        callbackTypes[14] = CallbackType.SwapV2; // hakuswapCall
        callbackTypes[15] = CallbackType.SwapV2; // canaryCall
        callbackTypes[16] = CallbackType.SwapV2; // complusV2Call
        callbackTypes[17] = CallbackType.SwapV2; // swapCall
        callbackTypes[18] = CallbackType.SwapV2; // VaporDEXCall
        callbackTypes[19] = CallbackType.SwapV2; // soulswapCall
        callbackTypes[20] = CallbackType.SwapV3; // swapCallback
        callbackTypes[21] = CallbackType.SwapV2; // partyCall
        callbackTypes[22] = CallbackType.SwapV3; // elkDexV3SwapCallback
        callbackTypes[23] = CallbackType.SwapV2; // contextCall
        callbackTypes[24] = CallbackType.SwapV2; // flashLiquidityCall
        callbackTypes[25] = CallbackType.SwapV2; // pancakeCall
        callbackTypes[26] = CallbackType.SwapV2; // StormCall
        callbackTypes[27] = CallbackType.SwapV2; // AvaxPadSwapCall
        callbackTypes[28] = CallbackType.SwapV2; // moeCall
        callbackTypes[29] = CallbackType.SwapV2; // oliveCall
        callbackTypes[30] = CallbackType.SwapV3; // elixirSwapCallback
        callbackTypes[31] = CallbackType.SwapV2; // baguetteCall
        callbackTypes[32] = CallbackType.SwapV2; // ruggyCall
        callbackTypes[33] = CallbackType.SwapV2; // Call
        callbackTypes[34] = CallbackType.SwapV2; // lpCall
        callbackTypes[35] = CallbackType.SwapV2; // 4a639df4
        callbackTypes[36] = CallbackType.SwapV2; // apexCall
        callbackTypes[37] = CallbackType.SwapV2; // viralswapCall
        callbackTypes[38] = CallbackType.SwapV2; // unifiCall
        callbackTypes[39] = CallbackType.SwapV2; // ovxCall
        callbackTypes[40] = CallbackType.SwapV2; // whaleswapCall
        callbackTypes[41] = CallbackType.SwapV2; // d66af394
        callbackTypes[42] = CallbackType.SwapV2; // AzurSwapV2Call
        callbackTypes[43] = CallbackType.SwapV2; // zeroCall
        callbackTypes[44] = CallbackType.SwapV2; // CandyManCall
        callbackTypes[45] = CallbackType.SwapV2; // otbSwapCall
        callbackTypes[46] = CallbackType.SwapV2; // zswapCall
        callbackTypes[47] = CallbackType.SwapV2; // hook
        callbackTypes[48] = CallbackType.SwapV3; // hook
        callbackTypes[49] = CallbackType.SwapV3; // uniswapV3Call
        callbackTypes[50] = CallbackType.SwapV2; // swapV2Call
        callbackTypes[51] = CallbackType.SwapV3; // swapV3Call
        callbackTypes[52] = CallbackType.SwapV3; // v3SwapCallback
        callbackTypes[53] = CallbackType.SwapV2; // v2Call

        // Define V2 factory addresses
        address[] memory v2Factories = new address[](72);
        v2Factories[0] = 0x9Ad6C38BE94206cA50bb0d90783181662f0Cfa10; // joeCall
        v2Factories[1] = 0xF16784dcAf838a3e16bEF7711a62D12413c39BD1; // arenaCall
        v2Factories[2] = 0xefa94DE7a4656D787667C749f7E1223D71E9FD88; // pangolinCall
        v2Factories[3] = 0xc35DADB65012eC5796536bD9864eD8773aBc74C4; // uniswapV2Call
        v2Factories[4] = 0x9C60C867cE07a3c403E2598388673C10259EC768; // sicleCall
        v2Factories[5] = 0xf77ca9B635898980fb219b4F4605C50e4ba58afF; // uniswapV2Call
        v2Factories[6] = 0x9e5A52f57b3038F1B8EeE45F28b3C1967e22799C; // uniswapV2Call
        v2Factories[7] = 0xe0C1bb6DF4851feEEdc3E14Bd509FEAF428f7655; // lydiaCall
        v2Factories[8] = 0x8e6F4Af0B6c26d16feBdD6f28FA7C694bD49c6BF; // uniswapV2Call
        v2Factories[9] = 0xD9362AA8E0405C93299C573036E7FB4ec3bE1240; // alligatorCall
        v2Factories[10] = 0x2131Bdb0E0B451BC1C5A53F2cBC80B16D43634Fa; // forwardCall
        v2Factories[11] = 0xA0FbfDa09B8815Dd42dDC70E4f9fe794257CD9B6; // uniswapV2Call
        v2Factories[12] = 0x091d35d7F63487909C863001ddCA481c6De47091; // elkCall
        v2Factories[13] = 0x26B42c208D8a9d8737A2E5c9C57F4481484d4616; // uniswapV2Call
        v2Factories[14] = 0x58C8CD291Fa36130119E6dEb9E520fbb6AcA1c3a; // yetiswapCall
        v2Factories[15] = 0xAAA16c016BF556fcD620328f0759252E29b1AB57; // hook
        v2Factories[16] = 0x2Db46fEB38C57a6621BCa4d97820e1fc1de40f41; // hakuswapCall
        v2Factories[17] = 0x814EBF333BDaF1D2d364c22a1e2400a812f1F850; // uniswapV2Call
        v2Factories[18] = 0xCFBA329d49C24b70F3a8b9CC0853493d4645436b; // canaryCall
        v2Factories[19] = 0x5C02e78A3969D0E64aa2CFA765ACc1d671914aC0; // complusV2Call
        v2Factories[20] = 0xa98ea6356A316b44Bf710D5f9b6b4eA0081409Ef; // swapCall
        v2Factories[21] = 0xC009a670E2B02e21E7e75AE98e254F467f7ae257; // VaporDEXCall
        v2Factories[22] = 0x0c6A0061F9D0afB30152b8761a273786e51bec6d; // uniswapV2Call
        v2Factories[23] = 0x5BB2a9984de4a69c05c996F7EF09597Ac8c9D63a; // soulswapCall
        v2Factories[24] = 0x634e02EB048eb1B5bDDc0CFdC20D34503E9B362d; // hook
        v2Factories[25] = 0x16871f3c042a9b0467F8166Dbe6CdDc6EC557a74; // uniswapV2Call
        v2Factories[26] = 0x7cFd6F0fB0802dB028d461ca25dAa0bA863a1F45; // uniswapV2Call
        v2Factories[27] = 0x045D720873f0260e23DA812501a7c5930E510aA4; // uniswapV2Call
        v2Factories[28] = 0x58A08bc28f3E8dab8Fb2773D8f243bC740398b09; // partyCall
        v2Factories[29] = 0x7009b3619d5ee60d0665BA27Cf85eDF95fd8Ad01; // uniswapV2Call
        v2Factories[30] = 0x9F0e80Ac5E09Dd1E37b40E8CDd749768FEAD43EB; // contextCall
        v2Factories[31] = 0x6e553d5f028bD747a27E138FA3109570081A23aE; // flashLiquidityCall
        v2Factories[32] = 0x042AF448582d0a3cE3CFa5b65c2675e88610B18d; // pancakeCall
        v2Factories[33] = 0x03C51A75A94b1cd075d6686846405dbdAfbDe390; // StormCall
        v2Factories[34] = 0x2fFa939c7dB9D4b4278713aDd0154b70cB82AA82; // AvaxPadSwapCall
        v2Factories[35] = 0x1051E74C859cc1e662C3AFa3F170103522A2e70f; // moeCall
        v2Factories[36] = 0xd9F58F79bcdFb5cF5E7741eb14Ca4060d32F2b21; // pancakeCall
        v2Factories[37] = 0xc7e37A28bB17EdB59E99d5485Dc8c51BC87aE699; // uniswapV2Call
        v2Factories[38] = 0x4Fe4D8b01A56706Bc6CaD26E8C59D0C7169976b3; // oliveCall
        v2Factories[39] = 0xDC0BD72CdeF330786BF6f331a6Aca539c0bb4EaB; // pancakeCall
        v2Factories[40] = 0x3587B8c0136c2C3605a9E5B03ab54Da3e4044b50; // baguetteCall
        v2Factories[41] = 0x6aB0C582b8e25b5B575c2797C4bef3Aa2827A58A; // uniswapV2Call
        v2Factories[42] = 0x2181B20a9aaE3C41Bbd7aDD59233Cc1B629a54eB; // pancakeCall
        v2Factories[43] = 0xA0BB8f9865f732C277d0C162249A4F6c157ae9D0; // uniswapV2Call
        v2Factories[44] = 0xE61A092225A6639844693626491200BE1333D5cb; // uniswapV2Call
        v2Factories[45] = 0x9a89fa373186ecC1Ccb3B9FE08335FFD9CDF35d8; // ruggyCall
        v2Factories[46] = 0xDfD34be29A8ffB58dEA78bD7A6340b89EBeEbBe2; // Call
        v2Factories[47] = 0x557Ade9f0c89d07C396B19C4efac102E4008736e; // lpCall
        v2Factories[48] = 0xE357f7d5652004D41A8E9405a5454eC94173e3E7; // 4a639df4
        v2Factories[49] = 0x21cadeb92c8BbFBEF98c3098846f0999209C3A97; // apexCall
        v2Factories[50] = 0x71255B66E1977be3b5e427256495E811774729f6; // viralswapCall
        v2Factories[51] = 0x3BCa0B7431f46050a99Ec3B1B7BB710B3eFd30DD; // uniswapV2Call
        v2Factories[52] = 0x7Ab5ac142799B0A3b6f95C27a1f2149EBCF5287d; // uniswapV2Call
        v2Factories[53] = 0x839547067bc885db205F5fA42dcFeEcDFf5A8530; // unifiCall
        v2Factories[54] = 0x231DF4D421f1F9e0AAe9bA3634a87EBC87A09c39; // joeCall
        v2Factories[55] = 0xE01cF83a89e8C32C0A9f91aCa7BfE554EBEE7141; // ovxCall
        v2Factories[56] = 0x45C2C071b503e734B4F05634E57D6997D39534A7; // uniswapV2Call
        v2Factories[57] = 0xABc26F8364cc0dD728Ac5c23fa40886fDa3dD121; // whaleswapCall
        v2Factories[58] = 0x38a83A88E6d77576083fD755D7387779eB291792; // d66af394
        v2Factories[59] = 0x26BA4cE017BcD67E2Ca9135BD58D3Fc9050FC58f; // AzurSwapV2Call
        v2Factories[60] = 0x2Ef422F30cdb7c5F1f7267AB5CF567A88974b308; // zeroCall
        v2Factories[61] = 0x3A9c3398D7BFE6149a5580D901B1f57b1c7d3ec0; // hakuswapCall
        v2Factories[62] = 0xA22FFF80baEF689976C55dabb193becdf023B6B9; // sicleCall
        v2Factories[63] = 0x8c7437A3a5882b197970D4351A9341fb9E0BFe39; // uniswapV2Call
        v2Factories[64] = 0xDE105B2137045E8B7eA28EA8aB98eA1f859a6562; // CandyManCall
        v2Factories[65] = 0xCdF5EBcFB2B9608Ee81Ff043100aBBc45c9E4599; // otbSwapCall
        v2Factories[66] = 0x87033126710CCa6d51C1A6A4f8a13f42Ef12E434; // uniswapV2Call
        v2Factories[67] = 0xDeC9231b2492ccE6BA01376E2cbd2bd821150e8C; // uniswapV2Call
        v2Factories[68] = 0xcDE3F9e6D452be6d955B1C7AaAEE3cA397EAc469; // zswapCall
        v2Factories[69] = 0x5Ca135cB8527d76e932f34B5145575F9d8cbE08E; // uniswapV2Call
        v2Factories[70] = 0xaC7B7EaC8310170109301034b8FdB75eCa4CC491; // icecreamCall
        v2Factories[71] = address(1); // fraxtalCall

        // Define V2 fees
        uint256[] memory v2Fees = new uint256[](72);
        v2Fees[0] = 30; // joeCall
        v2Fees[1] = 0; // arenaCall
        v2Fees[2] = 30; // pangolinCall
        v2Fees[3] = 30; // uniswapV2Call
        v2Fees[4] = 30; // sicleCall
        v2Fees[5] = 30; // uniswapV2Call
        v2Fees[6] = 30; // uniswapV2Call
        v2Fees[7] = 20; // lydiaCall
        v2Fees[8] = 30; // uniswapV2Call
        v2Fees[9] = 30; // alligatorCall
        v2Fees[10] = 20; // forwardCall
        v2Fees[11] = 10; // uniswapV2Call
        v2Fees[12] = 30; // elkCall
        v2Fees[13] = 30; // uniswapV2Call
        v2Fees[14] = 30; // yetiswapCall
        v2Fees[15] = 50; // hook
        v2Fees[16] = 20; // hakuswapCall
        v2Fees[17] = 50; // uniswapV2Call
        v2Fees[18] = 30; // canaryCall
        v2Fees[19] = 30; // complusV2Call
        v2Fees[20] = 10; // swapCall
        v2Fees[21] = 29; // VaporDEXCall
        v2Fees[22] = 30; // uniswapV2Call
        v2Fees[23] = 20; // soulswapCall
        v2Fees[24] = 20; // hook
        v2Fees[25] = 30; // uniswapV2Call
        v2Fees[26] = 30; // uniswapV2Call
        v2Fees[27] = 70; // uniswapV2Call
        v2Fees[28] = 20; // partyCall
        v2Fees[29] = 30; // uniswapV2Call
        v2Fees[30] = 30; // contextCall
        v2Fees[31] = 30; // flashLiquidityCall
        v2Fees[32] = 50; // pancakeCall
        v2Fees[33] = 25; // StormCall
        v2Fees[34] = 25; // AvaxPadSwapCall
        v2Fees[35] = 0; // moeCall
        v2Fees[36] = 50; // pancakeCall
        v2Fees[37] = 50; // uniswapV2Call
        v2Fees[38] = 20; // oliveCall
        v2Fees[39] = 50; // pancakeCall
        v2Fees[40] = 30; // baguetteCall
        v2Fees[41] = 30; // uniswapV2Call
        v2Fees[42] = 25; // pancakeCall
        v2Fees[43] = 50; // uniswapV2Call
        v2Fees[44] = 20; // uniswapV2Call
        v2Fees[45] = 20; // ruggyCall
        v2Fees[46] = 30; // Call
        v2Fees[47] = 30; // lpCall
        v2Fees[48] = 50; // 4a639df4
        v2Fees[49] = 20; // apexCall
        v2Fees[50] = 50; // viralswapCall
        v2Fees[51] = 50; // uniswapV2Call
        v2Fees[52] = 50; // uniswapV2Call
        v2Fees[53] = 50; // unifiCall
        v2Fees[54] = 50; // joeCall
        v2Fees[55] = 30; // ovxCall
        v2Fees[56] = 50; // uniswapV2Call
        v2Fees[57] = 25; // whaleswapCall
        v2Fees[58] = 50; // d66af394
        v2Fees[59] = 20; // AzurSwapV2Call
        v2Fees[60] = 50; // zeroCall
        v2Fees[61] = 10; // hakuswapCall
        v2Fees[62] = 30; // sicleCall
        v2Fees[63] = 30; // uniswapV2Call
        v2Fees[64] = 20; // CandyManCall
        v2Fees[65] = 20; // otbSwapCall
        v2Fees[66] = 30; // uniswapV2Call
        v2Fees[67] = 20; // uniswapV2Call
        v2Fees[68] = 25; // zswapCall
        v2Fees[69] = 30; // uniswapV2Call
        v2Fees[70] = 0; // icecreamCall
        v2Fees[71] = 50; // hook
        UniversalRouterCustomFee universalRouter = new UniversalRouterCustomFee(
            sigs,
            callbackTypes,
            v2Factories,
            v2Fees
        );

        console2.log("UniversalRouter deployed at", address(universalRouter));
    }
}
