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
        bytes4[] memory sigs = new bytes4[](58); // V2
        sigs[0] = bytes4(
            keccak256("pancakeCall(address,uint256,uint256,bytes)")
        );
        sigs[1] = bytes4(
            keccak256("BiswapCall(address,uint256,uint256,bytes)")
        );
        sigs[2] = bytes4(keccak256("babyCall(address,uint256,uint256,bytes)"));
        sigs[3] = bytes4(
            keccak256("fstswapCall(address,uint256,uint256,bytes)")
        );
        sigs[4] = bytes4(
            keccak256("nomiswapCall(address,uint256,uint256,bytes)")
        );
        sigs[5] = bytes4(
            keccak256("swapV2Call(address,uint256,uint256,bytes)")
        );
        sigs[6] = bytes4(
            keccak256("waultSwapCall(address,uint256,uint256,bytes)")
        );
        sigs[7] = bytes4(
            keccak256("uniswapV2Call(address,uint256,uint256,bytes)")
        );
        sigs[8] = bytes4(
            keccak256("squadswapCall(address,uint256,uint256,bytes)")
        );
        sigs[9] = bytes4(
            keccak256("BabyDogeCall(address,uint256,uint256,bytes)")
        );
        sigs[10] = bytes4(
            keccak256("definixCall(address,uint256,uint256,bytes)")
        );
        sigs[11] = bytes4(
            keccak256("orionpoolV2Call(address,uint256,uint256,bytes)")
        );
        sigs[12] = bytes4(keccak256("SwapCall(address,uint256,uint256,bytes)"));
        sigs[13] = bytes4(
            keccak256("AlitaSwapCall(address,uint256,uint256,bytes)")
        );
        sigs[14] = bytes4(
            keccak256("coinswapCall(address,uint256,uint256,bytes)")
        );
        sigs[15] = bytes4(keccak256("ammCall(address,uint256,uint256,bytes)"));
        sigs[16] = bytes4(keccak256("dexCall(address,uint256,uint256,bytes)"));
        sigs[17] = bytes4(
            keccak256("cheeseswapCall(address,uint256,uint256,bytes)")
        );
        sigs[18] = bytes4(keccak256("gibxCall(address,uint256,uint256,bytes)"));
        sigs[19] = bytes4(
            keccak256("globalCall(address,uint256,uint256,bytes)")
        );
        sigs[20] = bytes4(
            keccak256("BSCswapCall(address,uint256,uint256,bytes)")
        );
        sigs[21] = bytes4(
            keccak256("W3swapCall(address,uint256,uint256,bytes)")
        );
        sigs[22] = bytes4(
            keccak256("pantherCall(address,uint256,uint256,bytes)")
        );
        sigs[23] = bytes4(
            keccak256("stableXCall(address,uint256,uint256,bytes)")
        );
        sigs[24] = bytes4(
            keccak256("QiaoswapV2Call(address,uint256,uint256,bytes)")
        );
        sigs[25] = bytes4(
            keccak256("DooarSwapV2Call(address,uint256,uint256,bytes)")
        );
        sigs[26] = bytes4(
            keccak256("wardenCall(address,uint256,uint256,bytes)")
        );
        sigs[27] = bytes4(keccak256("FinsCall(address,uint256,uint256,bytes)"));
        sigs[28] = bytes4(
            keccak256("fastswapCall(address,uint256,uint256,bytes)")
        );
        sigs[29] = bytes4(keccak256("jwapCall(address,uint256,uint256,bytes)"));
        sigs[30] = bytes4(
            keccak256("latteSwapCall(address,uint256,uint256,bytes)")
        );
        sigs[31] = bytes4(
            keccak256("wakandaCall(address,uint256,uint256,bytes)")
        );
        sigs[32] = bytes4(keccak256("cafeCall(address,uint256,uint256,bytes)"));
        sigs[33] = bytes4(keccak256("cafeCall(address,uint256,uint256,bytes)"));
        sigs[34] = bytes4(
            keccak256("manyswapCall(address,uint256,uint256,bytes)")
        );
        sigs[35] = bytes4(keccak256("gcCall(address,uint256,uint256,bytes)"));
        sigs[36] = bytes4(
            keccak256("NarwhalswapCall(address,uint256,uint256,bytes)")
        );
        sigs[37] = bytes4(
            keccak256("WineryCall(address,uint256,uint256,bytes)")
        );
        sigs[38] = bytes4(
            keccak256("safeswapCall(address,uint256,uint256,bytes)")
        );
        sigs[39] = bytes4(
            keccak256("unicornxCall(address,uint256,uint256,bytes)")
        );
        sigs[40] = bytes4(
            keccak256("butterCall(address,uint256,uint256,bytes)")
        );
        sigs[41] = bytes4(
            keccak256("safeswapCall(address,uint256,uint256,bytes)")
        );
        sigs[42] = bytes4(
            keccak256("SwychCall(address,uint256,uint256,bytes)")
        );
        sigs[43] = bytes4(
            keccak256("kingsCall(address,uint256,uint256,bytes)")
        );
        sigs[44] = bytes4(
            keccak256("gravisCall(address,uint256,uint256,bytes)")
        );
        sigs[45] = bytes4(
            keccak256("StarverseCall(address,uint256,uint256,bytes)")
        );
        sigs[46] = bytes4(
            keccak256("planetCall(address,uint256,uint256,bytes)")
        );
        sigs[47] = bytes4(keccak256("hook(address,uint256,uint256,bytes)"));
        sigs[48] = bytes4(
            keccak256("jetswapCall(address,uint256,uint256,bytes)")
        );
        sigs[49] = bytes4(
            keccak256("YouSwapV2Call(address,uint256,uint256,bytes)")
        );
        sigs[50] = bytes4(
            keccak256("sphynxCall(address,uint256,uint256,bytes)")
        );
        sigs[51] = bytes4(keccak256("call(address,uint256,uint256,bytes)"));
        sigs[52] = bytes4(
            keccak256("boxswapCall(address,uint256,uint256,bytes)")
        );
        sigs[53] = bytes4(
            keccak256("luchowCall(address,uint256,uint256,bytes)")
        );
        sigs[54] = bytes4(
            keccak256("rimauCall(address,uint256,uint256,bytes)")
        );
        sigs[55] = bytes4(
            keccak256("cobraCall(address,uint256,uint256,bytes)")
        );
        sigs[56] = bytes4(
            keccak256("mochiCall(address,uint256,uint256,bytes)")
        );
        sigs[57] = bytes4(
            keccak256("crumbSwapCall(address,uint256,uint256,bytes)")
        );

        // Define callback types
        CallbackType[] memory callbackTypes = new CallbackType[](58);
        callbackTypes[0] = CallbackType.SwapV2; // ramsesV2SwapCallback
        callbackTypes[1] = CallbackType.SwapV2; // uniswapV3SwapCallback
        callbackTypes[2] = CallbackType.SwapV2; // joeCall
        callbackTypes[3] = CallbackType.SwapV2; // arenaCall
        callbackTypes[4] = CallbackType.SwapV2; // pangolinCall
        callbackTypes[5] = CallbackType.SwapV2; // uniswapV2Call
        callbackTypes[6] = CallbackType.SwapV2; // pangolinv3SwapCallback
        callbackTypes[7] = CallbackType.SwapV2; // sicleCall
        callbackTypes[8] = CallbackType.SwapV2; // donaswapV3SwapCallback
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
        callbackTypes[20] = CallbackType.SwapV2; // swapCallback
        callbackTypes[21] = CallbackType.SwapV2; // partyCall
        callbackTypes[22] = CallbackType.SwapV2; // elkDexV3SwapCallback
        callbackTypes[23] = CallbackType.SwapV2; // contextCall
        callbackTypes[24] = CallbackType.SwapV2; // flashLiquidityCall
        callbackTypes[25] = CallbackType.SwapV2; // pancakeCall
        callbackTypes[26] = CallbackType.SwapV2; // StormCall
        callbackTypes[27] = CallbackType.SwapV2; // AvaxPadSwapCall
        callbackTypes[28] = CallbackType.SwapV2; // moeCall
        callbackTypes[29] = CallbackType.SwapV2; // oliveCall
        callbackTypes[30] = CallbackType.SwapV2; // elixirSwapCallback
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
        callbackTypes[48] = CallbackType.SwapV2; // hook
        callbackTypes[49] = CallbackType.SwapV2; // uniswapV3Call
        callbackTypes[50] = CallbackType.SwapV2; // swapV2Call
        callbackTypes[51] = CallbackType.SwapV2; // swapV3Call
        callbackTypes[52] = CallbackType.SwapV2; // v3SwapCallback
        callbackTypes[53] = CallbackType.SwapV2; // v2Call
        callbackTypes[54] = CallbackType.SwapV2; // v2Call
        callbackTypes[55] = CallbackType.SwapV2; // v2Call
        callbackTypes[56] = CallbackType.SwapV2; // v2Call
        callbackTypes[57] = CallbackType.SwapV2; // v2Call

        // Define V2 factory addresses
        address[] memory v2Factories = new address[](72);
        v2Factories[0] = 0xcA143Ce32Fe78f1f7019d7d551a6402fC5350c73; // joeCall
        v2Factories[1] = 0x858E3312ed3A876947EA49d572A7C42DE08af7EE; // arenaCall
        v2Factories[2] = 0x0841BD0B734E4F5853f0dD8d7Ea041c241fb0Da6; // pangolinCall
        v2Factories[3] = 0xBCfCcbde45cE874adCB698cC183deBcF17952812; // uniswapV2Call
        v2Factories[4] = 0x86407bEa2078ea5f5EB5A52B2caA963bC1F889Da; // sicleCall
        v2Factories[5] = 0x9A272d734c5a0d7d84E0a892e891a553e8066dce; // uniswapV2Call
        v2Factories[6] = 0xd6715A8be3944ec72738F0BFDC739d48C3c29349; // uniswapV2Call
        v2Factories[7] = 0x3CD1C46068dAEa5Ebb0d3f55F6915B10648062B8; // lydiaCall
        v2Factories[8] = 0xB42E3FE71b7E0673335b3331B3e1053BD9822570; // uniswapV2Call
        v2Factories[9] = 0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6; // alligatorCall
        v2Factories[10] = 0x918Adf1f2C03b244823Cd712E010B6e3CD653DbA; // forwardCall
        v2Factories[11] = 0x4693B62E5fc9c0a45F89D62e6300a03C85f43137; // uniswapV2Call
        v2Factories[12] = 0x43eBb0cb9bD53A3Ed928Dd662095aCE1cef92D19; // elkCall
        v2Factories[13] = 0xE52cCf7B6cE4817449F2E6fA7efD7B567803E4b4; // uniswapV2Call
        v2Factories[14] = 0x73D9F93D53505cB8C4c7f952ae42450d9E859D10; // yetiswapCall
        v2Factories[15] = 0x35b4B2Fb6D3156E44A2CdD9006f81B0371B3D808; // hook
        v2Factories[16] = 0xc35DADB65012eC5796536bD9864eD8773aBc74C4; // hakuswapCall
        v2Factories[17] = 0xf0bc2E21a76513aa7CC2730C7A1D6deE0790751f; // uniswapV2Call
        v2Factories[18] = 0xC7a506ab3ac668EAb6bF9eCf971433D6CFeF05D9; // canaryCall
        v2Factories[19] = 0xC2D8d27F3196D9989aBf366230a47384010440c0; // complusV2Call
        v2Factories[20] = 0x907e8C7D471877b4742dA8aA53d257d0d565A47E; // swapCall
        v2Factories[21] = 0xb7E5848e1d0CB457f2026670fCb9BbdB7e9E039C; // VaporDEXCall
        v2Factories[22] = 0xdd538E4Fd1b69B7863E1F741213276A6Cf1EfB3B; // uniswapV2Call
        v2Factories[23] = 0x1D9F43a6195054313ac1aE423B1f810f593b6ac1; // soulswapCall
        v2Factories[24] = 0x97bCD9BB482144291D77ee53bFa99317A82066E8; // hook
        v2Factories[25] = 0x738B815eaDD06E0041b52B0C9d4F0d0D277B24bA; // uniswapV2Call
        v2Factories[26] = 0x98957ab49b8bc9f7ddbCfD8BcC83728085ecb238; // uniswapV2Call
        v2Factories[27] = 0xF238d267B3B1C85F2a95354251C20626bb7bc2A1; // uniswapV2Call
        v2Factories[28] = 0xCe8fd65646F2a2a897755A1188C04aCe94D2B8D0; // partyCall
        v2Factories[29] = 0xD04A80baeeF12fD7b1D1ee6b1f8ad354f81bc4d7; // uniswapV2Call
        v2Factories[30] = 0x670f55c6284c629c23baE99F585e3f17E8b9FC31; // contextCall
        v2Factories[31] = 0x94b4188D143b9dD6bd7083aE38A461FcC6AAd07E; // flashLiquidityCall
        v2Factories[32] = 0x918d7e714243F7d9d463C37e106235dCde294ffC; // pancakeCall
        v2Factories[33] = 0x6d8EDFf1B0a01F28516Eeee58EBF99FE977dB511; // StormCall
        v2Factories[34] = 0x1e895bFe59E3A5103e8B7dA3897d1F2391476f3c; // AvaxPadSwapCall
        v2Factories[35] = 0x3657952d7bA5A0A4799809b5B6fdfF9ec5B46293; // moeCall
        v2Factories[36] = 0x877Fe7F4e22e21bE397Cd9364fAFd4aF4E15Edb6; // pancakeCall
        v2Factories[37] = 0xe759Dd4B9f99392Be64f1050a6A8018f73B53a13; // uniswapV2Call
        v2Factories[38] = 0x59DA12BDc470C8e85cA26661Ee3DCD9B85247C88; // oliveCall
        v2Factories[39] = 0xd654CbF99F2907F06c88399AE123606121247D5C; // pancakeCall
        v2Factories[40] = 0x4DcE5Bdb81B8D5EdB66cA1b8b2616A8E0Dd5f807; // baguetteCall
        v2Factories[41] = 0x184411227f47F614e49cfab277D0F3Bfc65D2316; // uniswapV2Call
        v2Factories[42] = 0x3e708FdbE3ADA63fc94F8F61811196f1302137AD; // pancakeCall
        v2Factories[43] = 0xae52c26976E56e9f8829396489A4b7FfEbe8aAE9; // uniswapV2Call
        v2Factories[44] = 0xf4266b546a5153f2C9Cd8CD67B7260718172143f; // uniswapV2Call
        v2Factories[45] = 0xB9fA84912FF2383a617d8b433E926Adf0Dd3FEa1; // ruggyCall
        v2Factories[46] = 0x79C342FddBBF376cA6B4EFAc7aaA457D6063F8Cb; // Call
        v2Factories[47] = 0x4d05D0045df5562D6D52937e93De6Ec1FECDAd21; // lpCall
        v2Factories[48] = 0x71f843BD057d2eE39AE52186a33c3aFD1124805A; // 4a639df4
        v2Factories[49] = 0x957d1361F1929Daa61c7d41C8561559Cf58b13f3; // apexCall
        v2Factories[50] = 0x787557689775Df6791c729014C78ABAC6Cb8F632; // viralswapCall
        v2Factories[51] = 0x1Ba94C0851D96b2c0a01382Bf895B5b25361CcB2; // uniswapV2Call
        v2Factories[52] = 0x86A859773cf6df9C8117F20b0B950adA84e7644d; // uniswapV2Call
        v2Factories[53] = 0x80f112CD8Ac529d6993090A0c9a04E01d495BfBf; // unifiCall
        v2Factories[54] = 0x81Bb3E7b2448786a82FEC9fe49e311af683F6723; // joeCall
        v2Factories[55] = 0xDD3779945963a270652bc5bAfdC2b79B7e7428C8; // ovxCall
        v2Factories[56] = 0x4a3B76860C1b76f0403025485DE7bfa1F08C48fD; // uniswapV2Call
        v2Factories[57] = 0x20aB15EaAFB195DeE6a145e845a8e6066513357D; // whaleswapCall
        v2Factories[58] = 0xa053582601214FEb3778031a002135cbBB7DBa18; // d66af394
        v2Factories[59] = 0x0eb58E5c8aA63314ff5547289185cC4583DfCBD5; // AzurSwapV2Call
        v2Factories[60] = 0x137f34dF5bcDB30f5E858FC77CB7Ab60f8F7a09a; // zeroCall
        v2Factories[61] = 0x8BA1a4C24DE655136DEd68410e222cCA80d43444; // hakuswapCall
        v2Factories[62] = 0xF2Fb1b5Be475E7E1b3C31082C958e781f73a1712; // sicleCall
        v2Factories[63] = 0xDB984fd8371d07db9cBf4A48Eb9676b09B12161D; // uniswapV2Call
        v2Factories[64] = 0x7F6AD1d60De7a908A28a25DBe961ABb68747cEB3; // CandyManCall
        v2Factories[65] = 0xaF042b1B77240063bc713B9357c39ABedec1b691; // otbSwapCall
        v2Factories[66] = 0xA78AAc0C0551ab3470F40ff5A382f0CDbFA31B7b; // uniswapV2Call
        v2Factories[67] = 0xAF3bA99201485E14472353a10dd7392845826314; // uniswapV2Call
        v2Factories[68] = 0xcb4Ee9910811EdB5fF3fe0e3CE3A8cEd25E24079; // zswapCall
        v2Factories[69] = 0xCBac17919f7aad11E623Af4FeA98B10B84802eAc; // uniswapV2Call
        v2Factories[70] = 0x7ab906d0ff39AADa772Ec95829fb6A048f19d531; // icecreamCall
        v2Factories[71] = address(1); // fraxtalCall

        // Define V2 fees
        uint256[] memory v2Fees = new uint256[](72);
        v2Fees[0] = 25; // joeCall
        v2Fees[1] = 10; // arenaCall
        v2Fees[2] = 20; // pangolinCall
        v2Fees[3] = 20; // uniswapV2Call
        v2Fees[4] = 20; // sicleCall
        v2Fees[5] = 30; // uniswapV2Call
        v2Fees[6] = 10; // uniswapV2Call
        v2Fees[7] = 30; // lydiaCall
        v2Fees[8] = 20; // uniswapV2Call
        v2Fees[9] = 30; // alligatorCall
        v2Fees[10] = 20; // forwardCall
        v2Fees[11] = 0; // uniswapV2Call
        v2Fees[12] = 20; // elkCall
        v2Fees[13] = 30; // uniswapV2Call
        v2Fees[14] = 30; // yetiswapCall
        v2Fees[15] = 25; // hook
        v2Fees[16] = 30; // hakuswapCall
        v2Fees[17] = 20; // uniswapV2Call
        v2Fees[18] = 20; // canaryCall
        v2Fees[19] = 20; // complusV2Call
        v2Fees[20] = 0; // swapCall
        v2Fees[21] = 25; // VaporDEXCall
        v2Fees[22] = 20; // uniswapV2Call
        v2Fees[23] = 20; // soulswapCall
        v2Fees[24] = 30; // hook
        v2Fees[25] = 30; // uniswapV2Call
        v2Fees[26] = 10; // uniswapV2Call
        v2Fees[27] = 9; // uniswapV2Call
        v2Fees[28] = 30; // partyCall
        v2Fees[29] = 3; // uniswapV2Call
        v2Fees[30] = 20; // contextCall
        v2Fees[31] = 30; // flashLiquidityCall
        v2Fees[32] = 6; // pancakeCall
        v2Fees[33] = 30; // StormCall
        v2Fees[34] = 100; // AvaxPadSwapCall
        v2Fees[35] = 30; // moeCall
        v2Fees[36] = 25; // pancakeCall
        v2Fees[37] = 25; // uniswapV2Call
        v2Fees[38] = 20; // oliveCall
        v2Fees[39] = 30; // pancakeCall
        v2Fees[40] = 25; // baguetteCall
        v2Fees[41] = 20; // uniswapV2Call
        v2Fees[42] = 20; // pancakeCall
        v2Fees[43] = 10; // uniswapV2Call
        v2Fees[44] = 20; // uniswapV2Call
        v2Fees[45] = 30; // ruggyCall
        v2Fees[46] = 17; // Call
        v2Fees[47] = 20; // lpCall
        v2Fees[48] = 20; // 4a639df4
        v2Fees[49] = 10; // apexCall
        v2Fees[50] = 10; // viralswapCall
        v2Fees[51] = 30; // uniswapV2Call
        v2Fees[52] = 20; // uniswapV2Call
        v2Fees[53] = 25; // unifiCall
        v2Fees[54] = 20; // joeCall
        v2Fees[55] = 2; // ovxCall
        v2Fees[56] = 25; // uniswapV2Call
        v2Fees[57] = 25; // whaleswapCall
        v2Fees[58] = 25; // d66af394
        v2Fees[59] = 30; // AzurSwapV2Call
        v2Fees[60] = 30; // zeroCall
        v2Fees[61] = 10; // hakuswapCall
        v2Fees[62] = 30; // sicleCall
        v2Fees[63] = 30; // uniswapV2Call
        v2Fees[64] = 30; // CandyManCall
        v2Fees[65] = 20; // otbSwapCall
        v2Fees[66] = 20; // uniswapV2Call
        v2Fees[67] = 20; // uniswapV2Call
        v2Fees[68] = 25; // zswapCall
        v2Fees[69] = 20; // uniswapV2Call
        v2Fees[70] = 0; // icecreamCall
        v2Fees[71] = 50; // hook
        UniversalRouterCustomFee universalRouter = new UniversalRouterCustomFee(
            sigs,
            callbackTypes,
            v2Factories,
            v2Fees
        );

        bytes4[] memory sigsV3 = new bytes4[](6); // V2
        sigsV3[0] = bytes4(
            keccak256("uniswapV3SwapCallback(int256,int256,bytes)")
        );
        sigsV3[1] = bytes4(
            keccak256("pancakeV3SwapCallback(int256,int256,bytes)")
        );
        sigsV3[2] = bytes4(keccak256("algebraSwapCallback(int256,int256,bytes)"));
        sigsV3[3] = bytes4(keccak256("squadV3SwapCallback(int256,int256,bytes)"));
        sigsV3[4] = bytes4(keccak256("swapCallback(int256,int256,bytes)"));
        sigsV3[5] = bytes4(keccak256("dexV3SwapCallback(int256,int256,bytes)"));

        CallbackType[] memory callbackTypesV3 = new CallbackType[](6);
        callbackTypesV3[0] = CallbackType.SwapV3; // ramsesV2SwapCallback
        callbackTypesV3[1] = CallbackType.SwapV3; // uniswapV3SwapCallback
        callbackTypesV3[2] = CallbackType.SwapV3; // joeCall
        callbackTypesV3[3] = CallbackType.SwapV3; // arenaCall
        callbackTypesV3[4] = CallbackType.SwapV3; // pangolinCall
        callbackTypesV3[5] = CallbackType.SwapV3; // uniswapV2Call

        universalRouter.setApprovedSigs(sigsV3, callbackTypesV3);

        console2.log("UniversalRouter deployed at", address(universalRouter));
    }
}
