// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/src/Test.sol";
import {Constant} from "./Constant.t.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {UniversalRouter, CallbackType} from "../contracts/UniversalRouter.sol";
import {IUniswapV3Pool} from "./IUniswapV3Pool.sol";
import {UniversalRouterCustomFee} from "../contracts/UniversalRouterCustomFee.sol";
import {console2} from "forge-std/src/console2.sol";

contract ForkUniversalDexLoopingHookTest is Constant, Test {
    IUniswapV3Pool public pair;

    function setUp() public {
        vm.createSelectFork("https://rpc.ankr.com/xdc"); // Fork from mainnet

        pair = IUniswapV3Pool(
            payable(0xE97FC53e549Cd5e92F832b31350Fa9272eee34C4)
        );
    }

    function testCallback() public {
        pair.swap(address(this), true, 1000, 10000000000, abi.encode(0x123456));
    }
}

// [2025-05-12 21:15:46] FAILED Block: 15244989 | Profit: 0.000000000000085943 | Tx: 0x0176e1e1267b343f65c2e21724d585e07c5717d55ac7fd04cb7ccc931694dca6 | Source: 0xc1abcf46f5aa5af287f175ab066c165000bcf079ab09e5956732a64839aea7c5 | Amount: 0.000000000095435229 | Path: 0x919C1c267BC06a7039e03fcc2eF738525769109c->0x91098391cD135A95f775752F8cBf59286729a948 --> 0xc86c7C0eFbd6A49B35E8714C5f59D99De09A225b->0x26216b7b7dE80399b601b8217DA272b82d4f34cb
// [2025-05-12 21:10:38] FAILED Block: 15244937 | Profit: 0.000000000000051450 | Tx: 0x8bc6c8bab8df1d3815f9b92455ed29572d636d1cea39e18d19731968777afd23 | Source: 0x32f3001e1b285513a87f906ea3bba97aaaf8734f2213e1127f4a5d92fa569818 | Amount: 0.000000000073659887 | Path: 0x919C1c267BC06a7039e03fcc2eF738525769109c->0x91098391cD135A95f775752F8cBf59286729a948 --> 0xc86c7C0eFbd6A49B35E8714C5f59D99De09A225b->0x26216b7b7dE80399b601b8217DA272b82d4f34cb

/// @title Quoter Interface
/// @notice Supports quoting the calculated amounts from exact input or exact output swaps
/// @dev These functions are not marked view because they rely on calling non-view functions and reverting
/// to compute the result. They are also not gas efficient and should not be called on-chain.
interface IQuoter {
    /// @notice Returns the amount out received for a given exact input swap without executing the swap
    /// @param path The path of the swap, i.e. each token pair and the pool fee
    /// @param amountIn The amount of the first token to swap
    /// @return amountOut The amount of the last token that would be received
    function quoteExactInput(
        bytes memory path,
        uint256 amountIn
    ) external returns (uint256 amountOut);

    /// @notice Returns the amount out received for a given exact input but for a swap of a single pool
    /// @param tokenIn The token being swapped in
    /// @param tokenOut The token being swapped out
    /// @param fee The fee of the token pool to consider for the pair
    /// @param amountIn The desired input amount
    /// @param sqrtPriceLimitX96 The price limit of the pool that cannot be exceeded by the swap
    /// @return amountOut The amount of `tokenOut` that would be received
    function quoteExactInputSingle(
        address tokenIn,
        address tokenOut,
        uint24 fee,
        uint256 amountIn,
        uint160 sqrtPriceLimitX96
    ) external returns (uint256 amountOut);

    /// @notice Returns the amount in required for a given exact output swap without executing the swap
    /// @param path The path of the swap, i.e. each token pair and the pool fee. Path must be provided in reverse order
    /// @param amountOut The amount of the last token to receive
    /// @return amountIn The amount of first token required to be paid
    function quoteExactOutput(
        bytes memory path,
        uint256 amountOut
    ) external returns (uint256 amountIn);

    /// @notice Returns the amount in required to receive the given exact output amount but for a swap of a single pool
    /// @param tokenIn The token being swapped in
    /// @param tokenOut The token being swapped out
    /// @param fee The fee of the token pool to consider for the pair
    /// @param amountOut The desired output amount
    /// @param sqrtPriceLimitX96 The price limit of the pool that cannot be exceeded by the swap
    /// @return amountIn The amount required as the input for the swap in order to receive `amountOut`
    function quoteExactOutputSingle(
        address tokenIn,
        address tokenOut,
        uint24 fee,
        uint256 amountOut,
        uint160 sqrtPriceLimitX96
    ) external returns (uint256 amountIn);
}
