// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IUniswapV2Pair} from "../interfaces/UniswapV2/IUniswapV2Pair.sol";

library UniswapV2LibraryCustomFee {
    // given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    function getAmountIn(
        IUniswapV2Pair pair,
        address tokenIn,
        uint256 amountOut,
        uint256 fee,
        uint256 feeDenominator
    ) internal view returns (uint256 amountIn) {
        uint256 oneMinusFee = feeDenominator - fee;
        require(amountOut > 0, "Router: INSUFFICIENT_OUTPUT_AMOUNT");

        (uint256 reserveIn, uint256 reserveOut, ) = pair.getReserves();

        if (tokenIn != pair.token0()) {
            (reserveIn, reserveOut) = (reserveOut, reserveIn);
        }
        require(amountOut < reserveOut, "Router: INSUFFICIENT_LIQUIDITY");

        uint256 numerator = reserveIn * amountOut * feeDenominator;
        uint256 denominator = (reserveOut - amountOut) * oneMinusFee;
        amountIn = numerator / denominator + 1;
    }

    // given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    function getAmountOut(
        IUniswapV2Pair pair,
        address tokenIn,
        uint256 amountIn,
        uint256 fee,
        uint256 feeDenominator
    ) internal view returns (uint256 amountOut) {
        uint256 oneMinusFee = feeDenominator - fee;
        require(amountIn > 0, "Router: INSUFFICIENT_INPUT_AMOUNT");

        (uint256 reserveIn, uint256 reserveOut, ) = pair.getReserves();

        if (tokenIn != pair.token0()) {
            (reserveIn, reserveOut) = (reserveOut, reserveIn);
        }
        require(amountOut < reserveOut, "Router: INSUFFICIENT_LIQUIDITY");

        uint256 amountInWithFee = amountIn * oneMinusFee;
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = reserveIn * feeDenominator + amountInWithFee;
        amountOut = numerator / denominator;
    }
}
